// Product wording belongs to Atelier; protocol identifiers and user data stay literal.
;(() => {
  const brand = (text) =>
    text.replace(
      /(\{[^}]*\}|`[^`]*`|(?:https?:\/\/|\/)[^\s<>"']+)|\bmagpie\b/gi,
      (word, literal) => (literal ? word : 'Atelier'),
    )
  const translate = t
  t = (text, vars) => {
    let out = brand(translate(text))
    if (vars) for (const key in vars) out = out.split('{' + key + '}').join(vars[key])
    return out
  }
  const element = el
  el = (tag, cls, text) =>
    element(
      tag,
      cls,
      text === 'Remote magpie'
        ? 'Remote Atelier'
        : text === 'magpie' || text === 'usemagpie.ai'
          ? 'Atelier'
          : text,
    )
  const repository = 'https://github.com/zixin947/codex-atelier'
  const productLinks = {
    'https://usemagpie.ai': repository,
    'https://github.com/yetone/magpie': repository,
    'https://discord.gg/vGSnD3ZKQF': repository + '/issues',
  }
  const request = api
  api = (path, body, ...args) => {
    if (path === 'open' && body?.url) {
      const target =
        productLinks[body.url] ||
        (body.url.startsWith('https://github.com/yetone/magpie/issues/')
          ? repository + '/issues'
          : '')
      if (target) body = { ...body, url: target }
    }
    return request(path, body, ...args)
  }
  const settings = renderSettings
  renderSettings = (...args) => {
    const result = settings(...args)
    const link = document.querySelector('button[title="github.com/yetone/magpie"]')
    if (link) link.title = 'github.com/zixin947/codex-atelier'
    return result
  }
  const announce = status
  status = (message, ...args) => announce(brand(message), ...args)
  for (const method of ['log', 'info', 'warn', 'error']) {
    const log = console[method].bind(console)
    console[method] = (...args) =>
      log(...args.map((value) => (typeof value === 'string' ? brand(value) : value)))
  }
  const mark = 'M8 8H16V36H8Z M20 17H28V36H20Z M32 26H40V36H32Z'
  const logo = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 44 44"><rect width="44" height="44" fill="#20251e"/><path d="${mark}" fill="#faff00"/></svg>`
  const originalIcon = icon
  icon = (name, lazy) => {
    if (name !== 'magpie') return originalIcon(name, lazy)
    const node = element('span', 'ic')
    node.dataset.icon = 'atelier'
    node.innerHTML = logo
    return node
  }
  document.title = 'Atelier'
  document
    .querySelector('meta[name="apple-mobile-web-app-title"]')
    ?.setAttribute('content', 'Atelier')
  document
    .querySelector('link[rel="icon"]')
    ?.setAttribute('href', 'data:image/svg+xml,' + encodeURIComponent(logo))
  for (const node of document.querySelectorAll('.brand > span:not(.logo)'))
    node.textContent = 'Atelier'
  document.getElementById('bird')?.setAttribute('d', mark)
  const style = document.createElement('style')
  style.textContent = '.bird>.parts{display:none!important}.bird>.whole{display:block!important}'
  document.head.append(style)
  translateStatic()
})()
