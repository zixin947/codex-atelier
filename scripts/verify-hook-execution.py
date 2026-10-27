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
parser.add_argument('--bridge-binary', type=pathlib.Path, help='Built Atelier executable for the Desktop transport acceptance test')
parser.add_argument('--bridge-control-test', type=pathlib.Path, help='Rust test executable containing bridge_acceptance_control')
parser.add_argument('--mid-turn-probe', action='store_true', help='Reload the owner between two real tool calls in one active turn')
parser.add_argument('--hot-reload-probe', action='store_true', help='Compare helper writes, owner hot reload and per-turn plugin filtering')
args = parser.parse_args()
if not args.codex:
    parser.error('Codex binary not found')
ROOT = pathlib.Path(tempfile.mkdtemp(prefix='atelier-hook-runtime-')).resolve()
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
MIDTURN = False
BLOCKED = threading.Event()
RELEASE = threading.Event()

class LocalResponses(BaseHTTPRequestHandler):
    def log_message(self, *_): pass
    def do_POST(self):
        request = json.loads(self.rfile.read(int(self.headers.get('Content-Length','0'))))
        REPORT['requests'] += 1
        seq = REPORT['requests']
        items = request.get('input', [])
        last_user = max((i for i,x in enumerate(items) if x.get('role') == 'user'), default=-1)
        outputs = sum(x.get('type') == 'function_call_output' for x in items[last_user+1:])
        had_output = outputs > 0
        if MIDTURN and outputs == 1:
            BLOCKED.set()
            if not RELEASE.wait(15): REPORT['fixtureTimedOut'] = True
        names = [x.get('name') or x.get('function',{}).get('name') for x in request.get('tools',[])]
        tool = next((n for n in names if n in ['exec_command','shell_command','shell']), None)
        response_id = f'resp_probe_{seq}'
        if tool and (not had_output or MIDTURN and outputs == 1):
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
    def __init__(self, bridge=False):
        self.bridge=bridge
        self.q=queue.Queue();self.n=0
        process_env=ENV.copy()
        binary=CODEX
        if bridge:
            binary=str(args.bridge_binary.resolve())
            process_env.update(ATELIER_DESKTOP_BRIDGE='1',ATELIER_REAL_CODEX=CODEX)
        self.p=subprocess.Popen([binary,'app-server','--stdio'],cwd=ROOT/'work',env=process_env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.DEVNULL,text=True)
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
    def turn(self,thread,label,disabled=None):
        global STAGE
        STAGE=label;before=len(REPORT['hookRuns'])
        params={'threadId':thread,'input':[{'type':'text','text':'Hook execution probe '+label+'. Run the harmless test command if requested by the local fixture.','text_elements':[]}]}
        if disabled is not None: params['disabledPluginIds']=disabled
        result=self.call('turn/start',params)
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
        if self.p.poll() is None:
            if self.bridge:
                self.p.terminate()
                try: self.p.wait(timeout=3)
                except subprocess.TimeoutExpired: self.p.kill();self.p.wait()
            else: self.p.kill();self.p.wait()

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

def hot_reload_probe(ctl, hook_rows, hosts):
    global STAGE
    cases = {
        'plugin_write_helper': (3, 1),
        'plugin_batch_reload_helper': (3, 1),
        'plugin_batch_reload_owner': (0, 0),
        'hook_state_write_helper': (3, 1),
        'hook_state_batch_reload_owner': (0, 0),
        'per_turn_disabled_plugins': (0, 0),
    }
    REPORT['cases'] = []
    for case, (expected_after, expected_end) in cases.items():
        reset = [{'keyPath': f'plugins."{plugin}".enabled', 'value': True, 'mergeStrategy': 'replace'} for plugin in IDS]
        reset += [{'keyPath': f'hooks.state."{h["key"]}".enabled', 'value': True, 'mergeStrategy': 'replace'} for h in hook_rows if h.get('pluginId') in IDS]
        ctl.call('config/batchWrite', {'edits': reset, 'expectedVersion': revision(ctl)})
        runner = Rpc(); hosts.append(runner)
        thread = runner.start(); runner.turn(thread, case + '_positive')
        positive = [h for h in REPORT['hookRuns'] if h['stage'] == case + '_positive']
        assert len(positive) == 4 and all(h['status'] == 'completed' for h in positive)
        if case == 'plugin_write_helper':
            set_plugins(ctl, False)
        elif case.startswith('plugin_batch'):
            target = runner if case.endswith('owner') else ctl
            edits = [{'keyPath': f'plugins."{plugin}".enabled', 'value': False, 'mergeStrategy': 'replace'} for plugin in IDS]
            target.call('config/batchWrite', {'edits': edits, 'expectedVersion': revision(target), 'reloadUserConfig': True})
        elif case.startswith('hook_state'):
            target = runner if case.endswith('owner') else ctl
            edits = [{'keyPath': f'hooks.state."{h["key"]}".enabled', 'value': False, 'mergeStrategy': 'replace'} for h in hook_rows if h.get('pluginId') in IDS]
            target.call('config/batchWrite', {'edits': edits, 'expectedVersion': revision(target), 'reloadUserConfig': case.endswith('owner')})
            listed = ctl.call('hooks/list', {'cwds': [str(ROOT/'work')]})['data'][0]['hooks']
            assert all(h['enabled'] is False for h in listed if h.get('pluginId') in IDS)
        runner.turn(thread, case + '_after', IDS if case == 'per_turn_disabled_plugins' else None)
        after = [h for h in REPORT['hookRuns'] if h['stage'] == case + '_after']
        STAGE = case + '_session_end'
        runner.call('thread/archive', {'threadId': thread})
        deadline = time.monotonic() + .5
        while time.monotonic() < deadline:
            try: runner.event(runner.q.get(timeout=.1))
            except queue.Empty: pass
        ended = [h for h in REPORT['hookRuns'] if h['stage'] == case + '_session_end']
        row = {'case': case, 'afterExecutions': len(after), 'afterEvents': [(h['plugin'], h['eventName'], h['status']) for h in after], 'sessionEndExecutions': len(ended)}
        REPORT['cases'].append(row); print(json.dumps(row), flush=True)
        assert len(after) == expected_after and len(ended) == expected_end
        runner.close()
    assert len(REPORT['commandCompletions']) == 12 and all(c['exitCode'] == 0 for c in REPORT['commandCompletions'])
    assert all(h['status'] == 'completed' for h in REPORT['hookRuns'])

def mid_turn_probe(hosts):
    global MIDTURN, STAGE
    runner=Rpc();hosts.append(runner)
    thread=runner.start();runner.turn(thread,'positive')
    MIDTURN=True
    STAGE='midturn_before_reload'
    turn=runner.call('turn/start',{'threadId':thread,'input':[{'type':'text','text':'Mid-turn hot reload probe.','text_elements':[]}]})
    assert BLOCKED.wait(10), 'Fixture did not reach the pause after first real command'
    print('PAUSED AFTER FIRST COMMAND',flush=True)
    runner.call('config/batchWrite',{'edits':[{'keyPath':f'plugins."{plugin}".enabled','value':False,'mergeStrategy':'replace'} for plugin in IDS],'expectedVersion':revision(runner),'reloadUserConfig':True})
    assert not REPORT.get('fixtureTimedOut'), 'Owner reload blocked until the active turn resumed'
    STAGE='midturn_after_reload'
    RELEASE.set()
    deadline=time.monotonic()+20
    while True:
        v=runner.q.get(timeout=max(.01,deadline-time.monotonic()));runner.event(v)
        if v.get('method')=='turn/completed' and v.get('params',{}).get('threadId')==thread:
            assert v['params']['turn']['status']=='completed';break
    print(json.dumps({'afterReloadEvents':[(h['plugin'],h['eventName'],h['status']) for h in REPORT['hookRuns'] if h['stage']=='midturn_after_reload'],'commandCompletions':REPORT['commandCompletions']}),flush=True)
    STAGE='midturn_session_end'
    runner.call('thread/archive',{'threadId':thread})
    deadline=time.monotonic()+.5
    while time.monotonic()<deadline:
        try: runner.event(runner.q.get(timeout=.1))
        except queue.Empty: pass
    counts={stage:sum(h['stage']==stage for h in REPORT['hookRuns']) for stage in ['positive','midturn_before_reload','midturn_after_reload','midturn_session_end']}
    REPORT['midTurnCounts']=counts
    assert counts=={'positive':4,'midturn_before_reload':3,'midturn_after_reload':0,'midturn_session_end':0}, counts
    assert all(h['status']=='completed' for h in REPORT['hookRuns'])
    assert len(REPORT['commandCompletions'])==3 and all(c['exitCode']==0 for c in REPORT['commandCompletions'])

def bridge_probe(hosts):
    global MIDTURN, STAGE
    assert args.bridge_control_test, '--bridge-control-test is required'
    ENV.update(ATELIER_BRIDGE_DIRECTORY=str(ROOT/'bridge'), ATELIER_REAL_CODEX=CODEX,
               ATELIER_BRIDGE_ACCEPTANCE_ROOT=str(ROOT))
    (ROOT/'acceptance-marker').write_text('atelier-isolated-bridge-test')
    def control(action):
        env=ENV.copy();env['ATELIER_BRIDGE_ACCEPTANCE_ACTION']=action
        p=subprocess.run([str(args.bridge_control_test.resolve()), '--ignored', '--exact',
            'plugins::tests::bridge_acceptance_control','--nocapture'],env=env,
            stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,timeout=20)
        assert p.returncode==0, 'Native control failed: '+p.stdout[-1500:]+p.stderr[-500:]
        line=next(line.removeprefix('ATELIER_RESULT ') for line in p.stdout.splitlines() if line.startswith('ATELIER_RESULT '))
        REPORT.setdefault('nativeControl',[]).append({'action':action,'result':json.loads(line)})
        print('NATIVE CONTROL '+action,flush=True)
    runner=Rpc(bridge=True);hosts.append(runner)
    thread=runner.start();runner.turn(thread,'bridge_positive')
    assert REPORT['stages'][-1]['hookExecutions']==4
    descriptors=list((ROOT/'bridge').glob('*.json'))
    assert len(descriptors)==1
    descriptor=descriptors[0].read_text()
    control('disable')
    runner.turn(thread,'bridge_disabled_same_thread')
    assert REPORT['stages'][-1]['hookExecutions']==0
    control('conflict')
    control('enable')
    runner.turn(thread,'bridge_reenabled_same_thread')
    assert REPORT['stages'][-1]['hookExecutions']==3
    MIDTURN=True;STAGE='bridge_midturn_before';BLOCKED.clear();RELEASE.clear()
    runner.call('turn/start',{'threadId':thread,'input':[{'type':'text','text':'Mid-turn bridge acceptance.','text_elements':[]}]})
    assert BLOCKED.wait(10)
    deadline=time.monotonic()+3
    while not any(c['stage']=='bridge_midturn_before' for c in REPORT['commandCompletions']):
        runner.event(runner.q.get(timeout=max(.01,deadline-time.monotonic())))
    control('disable')
    assert not REPORT.get('fixtureTimedOut'), 'Native control blocked until turn end'
    STAGE='bridge_midturn_after';RELEASE.set()
    deadline=time.monotonic()+20
    while True:
        v=runner.q.get(timeout=max(.01,deadline-time.monotonic()));runner.event(v)
        if v.get('method')=='turn/completed' and v.get('params',{}).get('threadId')==thread:
            assert v['params']['turn']['status']=='completed';break
    STAGE='bridge_session_end';runner.call('thread/archive',{'threadId':thread})
    deadline=time.monotonic()+.5
    while time.monotonic()<deadline:
        try: runner.event(runner.q.get(timeout=.1))
        except queue.Empty: pass
    counts={stage:sum(h['stage']==stage for h in REPORT['hookRuns']) for stage in
        ['bridge_midturn_before','bridge_midturn_after','bridge_session_end']}
    REPORT['bridgeCounts']=counts
    assert counts=={'bridge_midturn_before':3,'bridge_midturn_after':0,'bridge_session_end':0},counts
    runner.close()
    assert not list((ROOT/'bridge').glob('*.json')), 'Descriptor was not removed on bridge exit'
    stale=ROOT/'bridge/stale.json';stale.write_text(descriptor);stale.chmod(0o600)
    control('stale');stale.unlink()
    MIDTURN=False
    fresh=Rpc();hosts.append(fresh);fresh.turn(fresh.start(),'bridge_disabled_new_thread')
    assert REPORT['stages'][-1]['hookExecutions']==0
    assert len(REPORT['commandCompletions'])==6 and all(c['exitCode']==0 for c in REPORT['commandCompletions'])
    assert all(h['status']=='completed' for h in REPORT['hookRuns'])

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
    if args.bridge_binary:
        bridge_probe(hosts)
    elif args.mid_turn_probe:
        mid_turn_probe(hosts)
    elif args.hot_reload_probe:
        hot_reload_probe(ctl, hook_rows, hosts)
    else:
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
