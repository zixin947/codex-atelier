"""Execute pinned plugin Hooks in disposable Codex sessions; never use user credentials.

macOS manual acceptance probe. Uses only the Python standard library, Git,
Node and the selected Codex binary. The response fixture is local, not a model.
"""
import argparse, os, sys, json, time, queue, threading, tempfile, subprocess, pathlib, shutil
from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--codex', default=shutil.which('codex'), help='Codex executable to test')
parser.add_argument('--source', type=pathlib.Path, default=pathlib.Path.home()/'Library/Application Support/com.codexatelier.desktop/behavior-plugins/marketplace', help='Clean pinned upstream clones installed by Atelier')
parser.add_argument('--output', type=pathlib.Path, required=True, help='Sanitized JSON evidence file')
args = parser.parse_args()
if not args.codex:
    parser.error('Codex binary not found')
ROOT = pathlib.Path(tempfile.mkdtemp(prefix='atelier-hook-runtime-'))
CODEX = args.codex
SOURCE = args.source
IDS = ['ponytail@atelier-companions', 'stop-that-shit@atelier-companions']
ENV = os.environ.copy()
ENV.update(CODEX_HOME=str(ROOT/'codex'), CLAUDE_CONFIG_DIR=str(ROOT/'claude'),
           XDG_CONFIG_HOME=str(ROOT/'xdg'), STS_RUNTIME_DATA=str(ROOT/'sts'), PONYTAIL_MAP='0')
for name in ['OPENAI_API_KEY', 'OPENAI_ACCESS_TOKEN', 'CHATGPT_ACCESS_TOKEN']:
    ENV.pop(name, None)
for name in ['HTTP_PROXY','HTTPS_PROXY','ALL_PROXY','http_proxy','https_proxy','all_proxy']:
    ENV.pop(name, None)
ENV['NO_PROXY'] = '127.0.0.1,localhost'
ENV['no_proxy'] = '127.0.0.1,localhost'
REPORT = {'cli': None, 'isolation': 'temporary CODEX_HOME and plugin state; loopback response fixture; no real model or credentials', 'sources': [], 'stages': [], 'hookRuns': [], 'commandCompletions': [], 'requests': 0, 'toolCalls': 0}
STAGE = 'setup'

class LocalResponses(BaseHTTPRequestHandler):
    def log_message(self, *_): pass
    def do_POST(self):
        request = json.loads(self.rfile.read(int(self.headers.get('Content-Length','0'))))
        REPORT['requests'] += 1
        seq = REPORT['requests']
        items = request.get('input', [])
        last_user = max((i for i,x in enumerate(items) if x.get('role') == 'user'), default=-1)
        had_output = any(x.get('type') == 'function_call_output' for x in items[last_user+1:])
        names = [x.get('name') or x.get('function',{}).get('name') for x in request.get('tools',[])]
        tool = next((n for n in names if n in ['exec_command','shell_command','shell']), None)
        response_id = f'resp_probe_{seq}'
        if tool and not had_output:
            args = {'cmd':'true','max_output_tokens':100} if tool=='exec_command' else {'command':'true'}
            if tool=='shell': args={'command':['/usr/bin/true']}
            item={'id':f'fc_{seq}','type':'function_call','call_id':f'call_probe_{seq}','name':tool,'arguments':json.dumps(args),'status':'completed'}
            REPORT['toolCalls'] += 1
        else:
            item={'id':f'msg_{seq}','type':'message','role':'assistant','status':'completed','content':[{'type':'output_text','text':'Local hook execution probe complete.','annotations':[]}]}
        events=[
          {'type':'response.created','response':{'id':response_id,'status':'in_progress','output':[]}},
          {'type':'response.output_item.added','output_index':0,'item':item},
          {'type':'response.output_item.done','output_index':0,'item':item},
          {'type':'response.completed','response':{'id':response_id,'status':'completed','output':[item],'usage':{'input_tokens':1,'output_tokens':1,'total_tokens':2}}}
        ]
        body=''.join('event: '+e['type']+'\ndata: '+json.dumps(e)+'\n\n' for e in events).encode()
        self.send_response(200); self.send_header('Content-Type','text/event-stream'); self.send_header('Content-Length',str(len(body))); self.end_headers(); self.wfile.write(body)

class Rpc:
    def __init__(self):
        self.q=queue.Queue();self.n=0
        self.p=subprocess.Popen([CODEX,'app-server','--stdio'],cwd=ROOT/'work',env=ENV,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.DEVNULL,text=True)
        threading.Thread(target=self.reader,daemon=True).start()
        self.call('initialize',{'clientInfo':{'name':'atelier_hook_execution_probe','version':'0.1.0'},'capabilities':{'experimentalApi':True}})
        self.send({'method':'initialized'})
    def reader(self):
        for line in self.p.stdout:
            try: self.q.put(json.loads(line))
            except ValueError: pass
    def send(self,v): self.p.stdin.write(json.dumps(v)+'\n');self.p.stdin.flush()
    def event(self,v):
        if v.get('method')=='item/completed':
            item=v.get('params',{}).get('item',{})
            if item.get('type')=='commandExecution':
                REPORT['commandCompletions'].append({'stage':STAGE,'exitCode':item.get('exitCode'),'status':item.get('status')})
        if v.get('method')=='hook/completed':
            p=v['params'];run=p['run'];path=run.get('sourcePath','')
            plugin=next((x for x in ['ponytail','stop-that-shit'] if '/'+x+'/' in path), 'other')
            entry={'stage':STAGE,'threadId':p.get('threadId'),'runId':run['id'],'plugin':plugin,'eventName':run['eventName'],'status':run['status'],'durationMs':run.get('durationMs'),'entries':[{'kind':x.get('kind')} for x in run.get('entries',[])]}
            REPORT['hookRuns'].append(entry)
    def call(self,m,params):
        self.n+=1;i=self.n;self.send({'id':i,'method':m,'params':params});deadline=time.monotonic()+20
        while True:
            v=self.q.get(timeout=max(.01,deadline-time.monotonic()));self.event(v)
            if v.get('id')==i:
                if 'error' in v: raise RuntimeError(m+': '+json.dumps(v['error']))
                return v.get('result')
    def turn(self,thread,label):
        global STAGE
        STAGE=label;before=len(REPORT['hookRuns'])
        result=self.call('turn/start',{'threadId':thread,'input':[{'type':'text','text':'Hook execution probe '+label+'. Run the harmless test command if requested by the local fixture.','text_elements':[]}]})
        turn_id=result.get('turn',{}).get('id');deadline=time.monotonic()+30
        while True:
            v=self.q.get(timeout=max(.01,deadline-time.monotonic()));self.event(v)
            if v.get('method')=='turn/completed' and v.get('params',{}).get('threadId')==thread:
                turn=v['params']['turn'];status=turn.get('status');break
            if v.get('method')=='item/commandExecution/requestApproval': raise RuntimeError('unexpected approval')
        row={'stage':label,'threadId':thread,'turnId':turn_id,'turnStatus':status,'hookExecutions':len(REPORT['hookRuns'])-before}
        REPORT['stages'].append(row);print(json.dumps(row),flush=True)
        if status!='completed': raise RuntimeError('Probe turn failed: '+json.dumps(turn.get('error')))
    def start(self):
        return self.call('thread/start',{'cwd':str(ROOT/'work'),'model':'atelier-local-probe','modelProvider':'atelier_probe','approvalPolicy':'never','sandbox':'workspace-write'})['thread']['id']
    def close(self):
        self.p.kill();self.p.wait()

def cli(*args):
    r=subprocess.run([CODEX,*args],env=ENV,cwd=ROOT/'work',stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=20)
    if r.returncode: raise RuntimeError('CLI failed: '+r.stderr.decode()[:200])
    return json.loads(r.stdout)
def revision(rpc):
    c=rpc.call('config/read',{'includeLayers':True})
    return next(x['version'] for x in c['layers'] if x['name']['type']=='user' and not x['name'].get('profile'))
def set_plugins(rpc,enabled):
    for plugin in IDS:
        rpc.call('config/value/write',{'keyPath':f'plugins."{plugin}".enabled','value':enabled,'mergeStrategy':'replace','expectedVersion':revision(rpc)})

server=ThreadingHTTPServer(('127.0.0.1',0),LocalResponses)
threading.Thread(target=server.serve_forever,daemon=True).start()
hosts=[]
try:
    (ROOT/'codex').mkdir();(ROOT/'work').mkdir();(ROOT/'market/.agents/plugins').mkdir(parents=True)
    (ROOT/'codex/config.toml').write_text(f'model = "atelier-local-probe"\nmodel_provider = "atelier_probe"\n[model_providers.atelier_probe]\nname = "Atelier local test fixture"\nbase_url = "http://127.0.0.1:{server.server_port}/v1"\nwire_api = "responses"\nrequires_openai_auth = false\nrequest_max_retries = 0\nstream_max_retries = 0\n')
    entries=[]
    catalog=json.loads((pathlib.Path(__file__).resolve().parent.parent/'integrations/behavior-plugins.json').read_text())
    for definition in catalog:
        name=definition['id']
        source=SOURCE/name
        commit=subprocess.check_output(['git','-C',str(source),'rev-parse','HEAD'],text=True).strip()
        dirty=subprocess.check_output(['git','-C',str(source),'status','--porcelain'],text=True).strip()
        manifest=json.loads((source/'.codex-plugin/plugin.json').read_text())
        assert commit==definition['commit'] and not dirty, 'Source must match clean pinned commit'
        assert manifest['version']==definition['version'] and manifest['name']==name
        REPORT['sources'].append({'plugin':name,'version':manifest['version'],'commit':commit})
        (ROOT/'market'/name).symlink_to(SOURCE/name,target_is_directory=True)
        entries.append({'name':name,'source':{'source':'local','path':'./'+name}})
    (ROOT/'market/.agents/plugins/marketplace.json').write_text(json.dumps({'name':'atelier-companions','plugins':entries}))
    REPORT['cli']=subprocess.check_output([CODEX,'--version'],env=ENV,text=True).strip()
    cli('plugin','marketplace','add',str(ROOT/'market'),'--json')
    for plugin in IDS: cli('plugin','add',plugin,'--json')
    ctl=Rpc();hosts.append(ctl)
    hook_rows=ctl.call('hooks/list',{'cwds':[str(ROOT/'work')]})['data'][0]['hooks']
    edits=[{'keyPath':f'hooks.state."{h["key"]}".trusted_hash','value':h['currentHash'],'mergeStrategy':'replace'} for h in hook_rows if h.get('pluginId') in IDS]
    ctl.call('config/batchWrite',{'edits':edits,'expectedVersion':revision(ctl)})
    trusts=ctl.call('hooks/list',{'cwds':[str(ROOT/'work')]})['data'][0]['hooks']
    REPORT['isolatedTrust']=[{'pluginId':h['pluginId'],'eventName':h['eventName'],'trustStatus':h['trustStatus']} for h in trusts if h.get('pluginId') in IDS]
    print(json.dumps({'isolatedTrustedHooks':len(REPORT['isolatedTrust'])}),flush=True)
    assert len(REPORT['isolatedTrust'])==7 and all(h['trustStatus']=='trusted' for h in REPORT['isolatedTrust'])
    runner=Rpc();hosts.append(runner)
    a=runner.start();runner.turn(a,'enabled_existing_positive_control')
    set_plugins(ctl,False)
    runner.turn(a,'disabled_existing_thread')
    b=runner.start();runner.turn(b,'disabled_new_thread_same_host')
    fresh=Rpc();hosts.append(fresh)
    c=fresh.start();fresh.turn(c,'disabled_new_thread_fresh_host')
    STAGE='disabled_existing_session_end'
    runner.call('thread/archive',{'threadId':a})
    until=time.monotonic()+2
    while time.monotonic()<until:
        try: runner.event(runner.q.get(timeout=.2))
        except queue.Empty: pass
    def observed(label):
        return {(h['plugin'],h['eventName']) for h in REPORT['hookRuns'] if h['stage']==label and h['status']=='completed'}
    assert {('ponytail','sessionStart'),('ponytail','userPromptSubmit'),('stop-that-shit','userPromptSubmit'),('stop-that-shit','preToolUse')} <= observed('enabled_existing_positive_control')
    assert {('ponytail','userPromptSubmit'),('stop-that-shit','userPromptSubmit'),('stop-that-shit','preToolUse')} <= observed('disabled_existing_thread')
    assert not any(h['stage']=='disabled_new_thread_same_host' for h in REPORT['hookRuns'])
    assert not any(h['stage']=='disabled_new_thread_fresh_host' for h in REPORT['hookRuns'])
    assert ('stop-that-shit','sessionEnd') in observed('disabled_existing_session_end')
    assert all(h['status']=='completed' for h in REPORT['hookRuns'])
    assert len(REPORT['commandCompletions'])==4 and all(c['exitCode']==0 for c in REPORT['commandCompletions']), 'Expected four real successful harmless commands'
    REPORT['result']='completed'
except Exception as exc:
    REPORT['result']='incomplete';REPORT['error']=str(exc);print('PROBE ERROR: '+str(exc),flush=True)
finally:
    for host in reversed(hosts): host.close()
    server.shutdown();server.server_close()
    out=args.output.resolve()
    out.parent.mkdir(parents=True,exist_ok=True)
    out.write_text(json.dumps(REPORT,indent=2));print('REPORT '+str(out),flush=True)
    shutil.rmtree(ROOT)
sys.exit(0 if REPORT.get('result')=='completed' else 1)
