import { spawnSync } from 'node:child_process'
import { existsSync, mkdirSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
const root = fileURLToPath(new URL('../', import.meta.url))
const dest = `${root}.toolchain`

if (existsSync(`${dest}/go/bin/go`)) {
  const v = spawnSync(`${dest}/go/bin/go`, ['version'], { encoding: 'utf8' })
  console.log(`已存在本地 Go：${v.stdout?.trim()}`)
  process.exit(0)
}

// 官方源在部分网络下 TLS 不稳定，按可用性依次尝试镜像。
const sources = [
  'https://mirrors.aliyun.com/golang/go1.24.5.darwin-arm64.tar.gz',
  'https://golang.google.cn/dl/go1.24.5.darwin-arm64.tar.gz',
  'https://dl.google.com/go/go1.24.5.darwin-arm64.tar.gz',
]
const tarball = '/tmp/atelier-go.tar.gz'
let ok = false
for (const url of sources) {
  console.log(`下载 ${url}`)
  const r = spawnSync('curl', ['-sL', '--max-time', '900', '-o', tarball, url], {
    stdio: 'inherit',
  })
  if (r.status === 0 && existsSync(tarball)) {
    ok = true
    break
  }
}
if (!ok) {
  console.error('下载 Go 失败；请手动安装后通过 GO_BIN 指定路径。')
  process.exit(1)
}
mkdirSync(dest, { recursive: true })
const untar = spawnSync('tar', ['-xzf', tarball, '-C', dest], { stdio: 'inherit' })
process.exit(untar.status ?? 1)
