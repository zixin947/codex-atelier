import { expect, it, vi } from 'vitest'
import branding from '../src/features/connections/native-branding.js?raw'

it('brands product templates while preserving interpolated data and protocol paths', () => {
  document.head.innerHTML = '<title>magpie</title><link rel="icon" href="icons/favicon.svg">'
  document.body.innerHTML =
    '<div class="brand"><span>magpie</span></div><p id="user-content">magpie project</p>'
  const element = (tag: string, cls = '', text = '') => {
    const node = document.createElement(tag)
    node.className = cls
    node.textContent = text
    return node
  }
  const status = vi.fn()
  const run = new Function(
    't',
    'el',
    'status',
    'icon',
    'translateStatic',
    'api',
    'renderSettings',
    'document',
    'console',
    branding + '\nreturn {t,el,status,icon,api}',
  )
  const api = vi.fn()
  const ui = run(
    (text: string) => text,
    element,
    status,
    vi.fn(),
    vi.fn(),
    api,
    vi.fn(),
    document,
    {
      log: vi.fn(),
      info: vi.fn(),
      warn: vi.fn(),
      error: vi.fn(),
    },
  )
  expect(ui.t('via magpie · {name}', { name: 'magpie project' })).toBe(
    'via Atelier · magpie project',
  )
  expect(ui.t('provider {magpie}')).toBe('provider {magpie}')
  expect(ui.t('/v1/magpie/usage and https://github.com/yetone/magpie')).toBe(
    '/v1/magpie/usage and https://github.com/yetone/magpie',
  )
  expect(ui.el('b', '', 'magpie').textContent).toBe('Atelier')
  expect(ui.el('div', 'name', 'Remote magpie').textContent).toBe('Remote Atelier')
  ui.status('magpie is stopped')
  expect(status).toHaveBeenCalledWith('Atelier is stopped')
  expect(ui.icon('magpie').dataset.icon).toBe('atelier')
  ui.api('open', { url: 'https://usemagpie.ai' })
  expect(api).toHaveBeenCalledWith('open', { url: 'https://github.com/zixin947/codex-atelier' })
  expect(document.title).toBe('Atelier')
  expect(document.querySelector('.brand')?.textContent).toBe('Atelier')
  expect(document.getElementById('user-content')?.textContent).toBe('magpie project')
})
