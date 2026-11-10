import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {Worker} from 'node:worker_threads';
import {pricingSchedule} from './pricing-schedule.mjs';
export function createInsightsService(config) {
  let cache=null,pending=null,lastKey='',worker=null,closed=false;
  function view() {
    const c=config.resolve(), monitored=c.setting.monitorSessions !== false;
    let auth={},authChangedAt=0,authSize=0;
    try{const file=path.join(config.codexHome,'auth.json'),stat=fs.statSync(file);authChangedAt=stat.mtimeMs;authSize=stat.size;if(stat.size<1024*1024)auth=JSON.parse(fs.readFileSync(file,'utf8'));}catch{}
    const subscribed=!c.key && new URL(c.baseUrl).origin==='https://api.openai.com' && (auth.auth_mode==='chatgpt' || !!auth.tokens?.access_token);
    // Re-login/rotation, including within the same account, establishes a new
    // observation boundary. Never include the credential itself in the cache key.
    const key=crypto.createHash('sha256').update(JSON.stringify([c.accountId,auth.tokens?.account_id||'',
      monitored,subscribed,auth.auth_mode||'',authChangedAt,authSize])).digest('hex');
    return {c,monitored,subscribed,authChangedAt,key};
  }
  async function get() {
    const now=Date.now(), {c,monitored,subscribed,authChangedAt,key}=view(), pricing=pricingSchedule(c,now);
    if(lastKey!==key){cache=null;lastKey=key;}
    if(!monitored || closed)return {ok:true,pricing,subscription:{available:false,windows:[],reason:'本机会话观测已关闭'},tokens:null};
    if(!cache || now-cache.at>30000){
      if(!pending || pending.key!==key) {
        void worker?.terminate();
        const job={key,promise:null};
        job.promise=new Promise(resolve=>{
        worker=new Worker(new URL('./insights-worker.mjs',import.meta.url),{workerData:{codexHome:config.codexHome,now,quotaAfterMs:authChangedAt}});
        const current=worker;let finished=false;
        const done=data=>{if(finished)return;finished=true;clearTimeout(timeout);void current.terminate();if(worker===current)worker=null;resolve(data);};
        const timeout=setTimeout(()=>done({error:'observation-timeout'}),8000);timeout.unref();
        current.once('message',done);current.once('error',()=>done({error:'local-observation-unavailable'}));current.once('exit',()=>done({error:'local-observation-unavailable'}));
        }).finally(()=>{if(pending===job)pending=null;});
        pending=job;
      }
      const data=await pending.promise;
      if (closed || key!==lastKey || key!==view().key) return {ok:true,pricing,tokens:null,
        subscription:{available:false,windows:[],observedAt:null,reason:'连接已变更，请刷新订阅额度'}};
      cache={at:now,data};
    }
    const data=cache?.data||{},at=Date.now();
    if (process.env.ATELIER_QUOTA_FILE) {
      try {
        const file=process.env.ATELIER_QUOTA_FILE;
        if(fs.statSync(file).size>64*1024)throw new Error('quota file too large');
        const shared=JSON.parse(fs.readFileSync(file,'utf8'));
        const fingerprint=crypto.createHash('sha256').update(fs.readFileSync(path.join(config.codexHome,'auth.json'))).digest('hex');
        if(shared.version!==1 || shared.authFingerprint!==fingerprint) {
          return {ok:true,pricing,tokens:data.tokens||null,subscription:{available:false,windows:[],reason:'当前账号已变化，请在 Atelier 重新检测并刷新额度'}};
        }
        const sub=shared.subscription;
        if(!Array.isArray(sub.windows) || sub.windows.length>2)throw new Error('invalid windows');
        const windows=sub.windows.map(w=>{
          if(![300,10080].includes(w.windowDurationMins) || !Number.isFinite(w.usedPercent) || w.usedPercent<0 || w.usedPercent>100 || !Number.isFinite(w.observedAt) || !Number.isFinite(w.resetsAt))throw new Error('invalid window');
          return {...w,stale:w.stale || at-w.observedAt>15*60000 || w.resetsAt<=at};
        });
        return {ok:true,pricing,tokens:data.tokens||null,subscription:{available:windows.length>0,windows,observedAt:sub.observedAt,reason:sub.reason,source:'atelier'},error:null};
      } catch {
        return {ok:true,pricing,tokens:data.tokens||null,subscription:{available:false,windows:[],reason:'Atelier 额度暂不可读，请打开 Atelier 并刷新额度'}};
      }
    }
    const windows=subscribed ? (data.windows||[]).filter(w=>Number.isFinite(w.observedAt) && w.observedAt>=authChangedAt)
      .map(w=>({...w,stale:at-w.observedAt>15*60000 || !Number.isFinite(w.resetsAt) || w.resetsAt<=at})) : [];
    const observedAt=windows.length ? Math.max(...windows.map(w=>w.observedAt)) : null;
    return {ok:true,pricing,tokens:data.tokens||null,subscription:{available:windows.length>0,windows,observedAt,
      reason:!subscribed?'当前连接不是可识别的 ChatGPT 订阅登录':windows.length?'来自本机会话的官方额度快照；不能换算为固定 token 配额':'尚未观测到订阅额度快照，请在订阅登录下使用 Codex 后刷新'},error:data.error||null};
  }
  return {get,close(){closed=true;void worker?.terminate();worker=null;cache=null;}};
}
