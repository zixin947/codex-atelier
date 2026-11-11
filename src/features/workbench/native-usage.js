// Route the unchanged upstream Usage UI to its ChatGPT-only backend.
;(() => {
  const tab = new URL(location.href).searchParams.get('tab') || 'usage'
  const scoped = (value) => {
    const url = new URL(value, location.href)
    if (
      url.protocol === location.protocol &&
      url.host === location.host &&
      url.pathname.startsWith('/api/') &&
      !url.pathname.startsWith('/api/atelier-chatgpt/')
    ) {
      url.pathname = '/api/atelier-chatgpt/' + url.pathname.slice(5)
    }
    return url.href
  }
  const fetch = window.fetch.bind(window)
  window.fetch = (input, init) =>
    fetch(input instanceof Request ? new Request(scoped(input.url), input) : scoped(input), init)
  if (['usage', 'requests', 'sessions', 'context'].includes(tab)) {
    // WKWebView can deny storage; that must never disable request scoping.
    try {
      localStorage.setItem('magpie.usageTab', tab)
    } catch {}
    document.addEventListener('DOMContentLoaded', () => {
      const select = () => {
        const buttons = document.querySelectorAll('#usageTab > button')
        if (buttons.length !== 4) return
        observer.disconnect()
        buttons[['usage', 'requests', 'sessions', 'context'].indexOf(tab)].click()
      }
      const observer = new MutationObserver(select)
      observer.observe(document.documentElement, { childList: true, subtree: true })
      select()
    })
  }
  // Upstream downloads click detached anchors, which never reach document.
  const click = HTMLAnchorElement.prototype.click
  HTMLAnchorElement.prototype.click = function () {
    this.href = scoped(this.href)
    return click.call(this)
  }
  document.addEventListener(
    'click',
    (event) => {
      const link = event.target.closest?.('a[href]')
      if (link) link.href = scoped(link.href)
    },
    true,
  )
})()
