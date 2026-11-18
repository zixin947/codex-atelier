// Runs only in the protected native management page, on the same origin as Magpie.
;(() => {
  const routes = new Map(),
    animations = new Map(),
    abort = new AbortController()
  let lastList = ''
  let seq = 0,
    loaded = false,
    owner = null,
    stopped = false,
    connected = false,
    selected = null,
    filter = 'all'
  playing = false
  cancelAnimationFrame(frame)
  cargo.replaceChildren()
  document.querySelector('header p').textContent =
    '实时路由 · 请求与响应是同一网关的两个阶段 · 货箱代表请求事件'
  document.querySelector('.controls').innerHTML =
    '<button id="live-refresh">重新连接</button><a href="?view=routing&atelierStyle=endfield&atelierContent=1&atelierBlueprint=off">原版详情与历史</a>'
  document.querySelector('footer > span:last-child').textContent =
    '仅显示网关已记录的数据；缺失项为 —'
  const details = document.createElement('section')
  details.style.cssText =
    'padding:20px;background:#f4f6ec;border:1px solid #a5af96;overflow-wrap:anywhere'
  details.innerHTML =
    '<h2 style="font-size:16px">请求记录</h2><button id="live-all">全部记录</button><div id="live-list"></div><pre id="live-detail" style="white-space:pre-wrap"></pre>'
  document.querySelector('main').append(details)
  const terminals = [...document.querySelectorAll('#industrial-detail .screen-text')]
  terminals.forEach((n) => (n.style.fontSize = '9px'))
  document.querySelectorAll('#scene text').forEach((n) => {
    if (n.textContent === '视觉演示 · 非生产运行状态') n.textContent = '实时网关 · 双中枢为阶段投影'
    if (n.textContent === '装饰线缆 · 无数据连接') n.textContent = '信号线 · 当前请求状态'
    if (n.textContent === 'BLUEPRINT / TRANSPORT DEMO') n.textContent = 'BLUEPRINT / LIVE ROUTING'
  })
  const cables = [...document.querySelectorAll('#industrial-detail path')].filter(
    (n) => n.getAttribute('stroke-dasharray') === '2 5',
  )
  const panels = [...document.querySelectorAll('#industrial-detail > g')].filter((g) =>
    g.querySelector('text')?.textContent.startsWith('维护面板'),
  )
  const cells = panels.map((panel) => {
    panel.querySelectorAll('path, rect:not(:first-child)').forEach((n) => n.remove())
    return [0, 1, 2].map((i) => {
      const t = document.createElementNS(ns, 'text')
      t.setAttribute('x', '10')
      t.setAttribute('y', String(38 + i * 20))
      t.setAttribute('font-size', '11')
      panel.append(t)
      return t
    })
  })
  const time = (value) => (Number.isFinite(value) ? `${(value / 1000).toFixed(2)} s` : '—')
  const short = (value) =>
    typeof value === 'string' ? (value.length > 9 ? value.slice(0, 8) + '…' : value) : '—'
  const write = (node, value) => {
    if (node.textContent !== value) node.textContent = value
  }
  function render() {
    const list = [...routes.values()].sort((a, b) => b.id - a.id),
      active = list.filter((r) => !r.done)
    const r = routes.get(selected) || active[0] || list[0],
      last = r?.tries?.at(-1)
    const color =
      !connected || !r || r.canceled || (r.done && !r.status)
        ? '#92998b'
        : !r.done
          ? '#ba9f37'
          : r.status >= 400
            ? '#bd6250'
            : '#7fa567'
    cables.forEach((n) => n.setAttribute('stroke', color))
    const status = !connected
      ? '数据连接中断'
      : !r
        ? '等待请求'
        : !r.done
          ? '处理中'
          : r.canceled
            ? '已取消'
            : r.status >= 400
              ? '失败'
              : r.status
                ? '完成'
                : '已结束 · 状态未提供'
    write(state, `${status} · 当前保留 ${list.length} 条记录 · 进行中 ${active.length}`)
    write(terminals[0], `${short(r?.agent)} / ${short(r?.model)}`)
    write(terminals[1], `${short(r?.provider)} / ${short(last?.model || r?.model)}`)
    write(document.getElementById('visibleCount'), String(active.length))
    write(document.getElementById('velocity'), `首字 ${time(r?.ttft)}`)
    const screen = document.querySelector('#overview .screen-small')
    write(screen, 'ACTIVE REQUESTS')
    document.querySelector('#overview text[x="420"]').textContent = '请求 / 进行中'
    document.querySelector('#overview text[y="503"]').textContent = `耗时 ${time(r?.ms)}`
    document.querySelector('#overview > text.tech').textContent = 'LIVE / 原生通道'
    document
      .querySelectorAll('#industrial-detail circle')
      .forEach((n) => n.setAttribute('fill', color))
    paths.forEach(
      (p, i) =>
        (p.style.stroke = !connected
          ? '#d1d5c6'
          : r?.status >= 400
            ? '#e2b4a5'
            : r && !r.done
              ? '#f1f58c'
              : '#dde3bd'),
    )
    const attempts = r?.tries || []
    if (cells[1])
      [
        `尝试 ${attempts.length}`,
        `状态 ${last?.status || '—'}`,
        `Token ${r?.tokens ?? '—'}`,
      ].forEach((v, i) => write(cells[1][i], v))
    const shown = list.filter(
      (r) =>
        filter === 'all' || (filter === 'errors' ? r.status >= 400 : (r.tries || []).length > 1),
    )
    const target = document.getElementById('live-list')
    const listKey = filter + JSON.stringify(shown.map((r) => [r.id, r.done, r.status, r.model]))
    if (lastList !== listKey) {
      lastList = listKey
      target.replaceChildren()
      if (!shown.length) target.textContent = '当前保留的记录中没有匹配项'
      for (const row of shown.slice(0, 20)) {
        const button = document.createElement('button')
        button.textContent = `#${row.id} ${short(row.agent)} · ${short(row.model)} · ${row.done ? row.status || '已结束' : '处理中'}`
        button.style.margin = '4px'
        button.onclick = () => {
          selected = row.id
          render()
        }
        target.append(button)
      }
    }
    document.getElementById('live-detail').textContent = r
      ? [
          `请求 #${r.id} · ${r.agent} → ${r.provider}`,
          `模型：${r.model}`,
          `总耗时：${time(r.ms)}　首字：${time(r.ttft)}　Token：${r.tokens ?? '—'}`,
          ...attempts.map(
            (t, i) =>
              `${i + 1}. ${t.model || '—'} · ${t.done ? t.status || '已结束' : '处理中'} · ${time(t.ms)}${t.fail ? ' · ' + t.fail : ''}`,
          ),
          r.error ? `错误：${r.error}` : '',
        ]
          .filter(Boolean)
          .join('\n')
      : '暂无请求记录'
  }
  function animate(now) {
    if (stopped) return
    for (const [id, item] of animations) {
      const r = routes.get(id)
      if (!r || !connected) {
        item.node.remove()
        animations.delete(id)
        continue
      }
      const outbound = lengths.slice(0, 4).reduce((a, b) => a + b, 0)
      if (r.done && !item.returned && now - item.start >= (outbound / 60) * 1000) {
        item.returned = true
        item.start = now
      }
      const span = item.returned ? total - outbound : outbound
      const distance = Math.min(span, ((now - item.start) / 1000) * 60)
      const at = position((item.returned ? outbound : 0) + distance)
      item.node.setAttribute('transform', `translate(${at.point.x} ${at.point.y}) scale(1.2)`)
      item.node.classList.toggle('back', item.returned)
      item.node.style.stroke = r.status >= 400 ? '#b24838' : ''
      item.node.setAttribute(
        'visibility',
        at.hidden || distance >= span || matchMedia('(prefers-reduced-motion: reduce)').matches
          ? 'hidden'
          : 'visible',
      )
      if (item.returned && distance >= span) {
        item.node.remove()
        animations.delete(id)
      }
    }
    frame = requestAnimationFrame(animate)
  }
  function clear() {
    routes.clear()
    for (const a of animations.values()) a.node.remove()
    animations.clear()
    selected = null
  }
  async function poll() {
    while (!stopped) {
      try {
        const response = await fetch(`/api/gateway/trace?after=${seq}${loaded ? '&wait=1' : ''}`, {
          signal: abort.signal,
        })
        if (!response.ok) throw new Error(`HTTP ${response.status}`)
        const data = await response.json()
        if (
          !Number.isFinite(data.seq) ||
          !Array.isArray(data.routes) ||
          !data.totals ||
          data.routes.some(
            (r) => !Number.isFinite(r.id) || typeof r.done !== 'boolean' || !Array.isArray(r.tries),
          )
        )
          throw new Error('路由数据格式无效')
        if (data.mine === false) throw new Error('当前网关未提供可观察的路由')
        if (data.seq < seq || (loaded && owner !== (data.by?.pid ?? null))) {
          clear()
          seq = 0
          loaded = false
          owner = data.by?.pid ?? null
          continue
        }
        owner = data.by?.pid ?? null
        connected = true
        for (const route of data.routes) {
          if (!routes.has(route.id) && (!route.done || loaded)) {
            const node = document.createElementNS(ns, 'g')
            node.setAttribute('class', 'cargo')
            const use = document.createElementNS(ns, 'use')
            use.setAttribute('href', '#crate')
            node.append(use)
            cargo.append(node)
            animations.set(route.id, { node, start: performance.now(), returned: false })
          }
          routes.set(route.id, route)
        }
        if (Array.isArray(data.ids))
          for (const [id, r] of routes) if (!r.done && !data.ids.includes(id)) routes.delete(id)
        for (const id of [...routes.keys()].sort((a, b) => b - a).slice(60)) routes.delete(id)
        seq = data.seq
        loaded = true
        if (cells[0])
          [
            `请求 ${data.totals.requests ?? '—'}`,
            `改道 ${data.totals.rerouted ?? '—'}`,
            `错误 ${data.totals.errors ?? '—'}`,
          ].forEach((v, i) => write(cells[0][i], v))
        render()
      } catch (error) {
        if (stopped) return
        connected = false
        render()
        state.textContent = `数据连接中断：${error.message}；画面保留上次记录`
        await new Promise((resolve) => setTimeout(resolve, 3000))
      }
    }
  }
  panels.forEach((p, i) => {
    p.style.cursor = 'pointer'
    p.setAttribute('role', 'button')
    p.setAttribute('tabindex', '0')
    p.setAttribute('aria-label', i ? '查看失败请求' : '查看改道请求')
    const select = () => {
      filter = i ? 'errors' : 'rerouted'
      render()
      details.scrollIntoView({ block: 'nearest' })
    }
    p.onclick = select
    p.onkeydown = (e) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault()
        select()
      }
    }
  })
  document.getElementById('live-all').onclick = () => {
    filter = 'all'
    render()
  }
  document.getElementById('live-refresh').onclick = () => location.reload()
  window.addEventListener(
    'pagehide',
    () => {
      stopped = true
      abort.abort()
      cancelAnimationFrame(frame)
    },
    { once: true },
  )
  document
    .querySelectorAll('#industrial-detail .screen-small')
    .forEach((n) => write(n, 'LIVE TELEMETRY'))
  render()
  frame = requestAnimationFrame(animate)
  poll()
})()
