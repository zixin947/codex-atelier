// @vitest-environment node
import fs from 'node:fs'
import path from 'node:path'
import { execFile } from 'node:child_process'
import { promisify } from 'node:util'
import { afterEach, expect, it } from 'vitest'
import { startBridge } from '../vendor/gpt-dragon-girl/runtime/bridge.mjs'

const source = fs.readFileSync('src-tauri/src/desktop_pets.rs', 'utf8')
const script = source.match(/const PET_STOP_SCRIPT: &str = r#"([\s\S]*?)"#;/)![1]!
let directory: string
let bridge: Awaited<ReturnType<typeof startBridge>> | undefined
let closing: Promise<void> | undefined

afterEach(async () => {
  await (closing ?? bridge?.close())
  bridge = undefined
  closing = undefined
  if (directory) fs.rmSync(directory, { recursive: true, force: true })
})

function createRuntime() {
  directory = fs.mkdtempSync('/tmp/pet-stop-')
  fs.mkdirSync(path.join(directory, 'runtime'))
  for (const file of ['process.mjs', 'bridge.mjs', 'paths.mjs']) {
    fs.copyFileSync(`vendor/gpt-dragon-girl/runtime/${file}`, path.join(directory, 'runtime', file))
  }
}

async function executeStop() {
  const result = await promisify(execFile)(
    process.execPath,
    ['--input-type=module', '-e', script],
    {
      cwd: directory,
      env: { ...process.env, WHALE_HOME: directory, CODEX_HOME: directory },
      timeout: 15000,
    },
  )
  return JSON.parse(result.stdout)
}

it('waits for delayed exit after one stop request using the actual IPC bridge', async () => {
  createRuntime()
  let requests = 0
  let probesAfterStop = 0
  bridge = await startBridge(
    {
      dispatch: async (route: string) => {
        if (route === '/api/stop') {
          requests++
          setTimeout(() => {
            closing = bridge!.close()
          }, 150)
        } else if (requests) probesAfterStop++
        return { status: 200, body: Buffer.from(JSON.stringify({ ok: true })) }
      },
    },
    { dataDir: directory },
  )
  expect(await executeStop()).toEqual({ ok: true, stopped: true })
  expect(requests).toBe(1)
  expect(probesAfterStop).toBeGreaterThan(0)
  expect(closing).toBeDefined()
  expect(fs.existsSync(path.join(directory, 'runtime.json'))).toBe(false)
})

it('succeeds when the pet has already stopped', async () => {
  createRuntime()
  expect(await executeStop()).toEqual({ ok: true, stopped: true })
})
