// Migrate preferences across the old and new native origins before starting the page.
window.atelierStorageReady = new Promise((resolve, reject) => {
  const origin = location.origin
  const legacy =
    origin === 'atelier://localhost'
      ? 'magpie://localhost'
      : origin === 'http://atelier.localhost'
        ? 'http://magpie.localhost'
        : ''
  const copy = (store, entries) => {
    let copied = 0
    for (const [key, value] of entries) {
      if (!key.startsWith('magpie.') || typeof value !== 'string') continue
      const target = 'atelier.' + key.slice(7)
      if (store.getItem(target) === null) {
        store.setItem(target, value)
        copied++
      }
    }
    return copied
  }
  copy(localStorage, Object.entries(localStorage))
  copy(sessionStorage, Object.entries(sessionStorage))
  if (!legacy || localStorage.getItem('atelier.identityVersion') === '1') {
    resolve()
    return
  }
  const start = () => {
    const frame = document.createElement('iframe')
    frame.hidden = true
    frame.src = legacy + '/atelier-storage-export'
    const finish = () => {
      clearTimeout(timeout)
      window.removeEventListener('message', receive)
      frame.remove()
    }
    const receive = (event) => {
      if (
        event.origin !== legacy ||
        event.source !== frame.contentWindow ||
        event.data?.type !== 'atelier-storage-export'
      )
        return
      try {
        copy(localStorage, event.data.local)
        copy(sessionStorage, event.data.session)
        localStorage.setItem('atelier.identityVersion', '1')
        finish()
        resolve()
      } catch (error) {
        finish()
        reject(error)
      }
    }
    const timeout = setTimeout(() => {
      finish()
      reject(new Error('旧版设置读取超时，请重新打开页面'))
    }, 5000)
    window.addEventListener('message', receive)
    document.body.append(frame)
  }
  if (document.readyState === 'loading')
    document.addEventListener('DOMContentLoaded', start, { once: true })
  else start()
})
