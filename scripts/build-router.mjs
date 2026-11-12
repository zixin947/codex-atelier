import { spawnSync } from 'node:child_process'
import { existsSync, mkdirSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
const root = fileURLToPath(new URL('../', import.meta.url))
mkdirSync(`${root}src-tauri/binaries`, { recursive: true })

// Go 工具链查找顺序：GO_BIN → PATH → 项目内 .toolchain → 常见安装位置。
// 项目内 .toolchain 由 scripts/install-go.mjs 准备，不污染系统环境。
const candidates = [
  process.env.GO_BIN,
  'go',
  `${root}.toolchain/go/bin/go`,
  '/opt/homebrew/bin/go',
  '/usr/local/go/bin/go',
].filter(Boolean)
const go = candidates.find((bin) => {
  if (bin === 'go') return spawnSync('go', ['version'], { stdio: 'ignore' }).status === 0
  return existsSync(bin)
})
if (!go) {
  console.error('未找到 Go 编译器。请运行 `node scripts/install-go.mjs`，或通过 GO_BIN 指定。')
  process.exit(1)
}

// vendor/magpie 的 go.mod 要求较新工具链，会自动下载；国内网络需要镜像。
const env = {
  ...process.env,
  GOPROXY: process.env.GOPROXY || 'https://goproxy.cn,direct',
  GOTOOLCHAIN: process.env.GOTOOLCHAIN || 'auto',
}
const result = spawnSync(
  go,
  ['build', '-trimpath', '-o', `${root}src-tauri/binaries/atelier-router`, './cmd/atelier-router'],
  { cwd: `${root}vendor/magpie`, stdio: 'inherit', env },
)
if (result.error) {
  console.error(`构建路由引擎失败：${result.error.message}`)
  process.exit(1)
}
process.exit(result.status ?? 1)
