import { spawnSync } from 'node:child_process'
import { mkdirSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
const root = fileURLToPath(new URL('../', import.meta.url))
mkdirSync(`${root}src-tauri/binaries`, { recursive: true })
const result = spawnSync(
  process.env.GO_BIN || 'go',
  ['build', '-trimpath', '-o', `${root}src-tauri/binaries/atelier-router`, './cmd/atelier-router'],
  { cwd: `${root}vendor/magpie`, stdio: 'inherit' },
)
if (result.error) {
  console.error('构建路由引擎需要 Go 1.26.8 或通过 GO_BIN 指定编译器。')
  process.exit(1)
}
process.exit(result.status ?? 1)
