import { readdirSync, statSync, existsSync, mkdirSync, cpSync } from 'node:fs'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import path from 'node:path'
if (process.platform !== 'darwin') process.exit(0)
const root = fileURLToPath(new URL('../', import.meta.url))
const source = path.join(root, 'vendor/endfield-native/Sources')
const output = path.join(root, 'src-tauri/binaries/hud-native')
const files = readdirSync(source)
  .filter((name) => name.endsWith('.swift'))
  .map((name) => path.join(source, name))
files.push(path.join(root, 'src-tauri/native-hud/AtelierHud.swift'))
mkdirSync(output, { recursive: true })
cpSync(
  path.join(root, 'vendor/endfield-native/Resources/WorldMap'),
  path.join(output, 'WorldMap'),
  { recursive: true },
)
cpSync(path.join(root, 'vendor/endfield-native/Resources/Fonts'), path.join(output, 'Fonts'), {
  recursive: true,
})
const library = path.join(output, 'libAtelierHUD.dylib')
if (
  !existsSync(library) ||
  files.some((file) => statSync(file).mtimeMs > statSync(library).mtimeMs)
) {
  const result = spawnSync(
    'xcrun',
    [
      'swiftc',
      '-D',
      'HUD_RELEASE',
      '-swift-version',
      '5',
      '-whole-module-optimization',
      '-O',
      '-parse-as-library',
      '-emit-library',
      '-module-name',
      'AtelierEndfield',
      '-o',
      library,
      ...files,
    ],
    { stdio: 'inherit' },
  )
  if (result.status !== 0) process.exit(result.status ?? 1)
}
