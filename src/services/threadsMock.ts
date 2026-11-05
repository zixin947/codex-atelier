import type {
  ThreadsService,
  ThreadRecord,
  ThreadPlan,
  ThreadBackup,
  ThreadDetail,
} from '../types/threads'
export function createThreadDemo(): ThreadsService {
  const now = Date.now()
  const titles = [
    '线程工作区 · 索引一致性检查',
    '终末地主题与控件动效',
    '账号额度后台刷新',
    '整理本地网关的路由边界',
    '会话导出与归档流程',
    '设计令牌与视觉校准',
    '补充跨模型兼容性说明',
    '检查历史记录完整性',
    '工作台布局优化',
    '为工具调用添加统计',
    '恢复旧版 Provider 记录',
    '研究项目工作区分组',
  ]
  let rows: ThreadRecord[] = titles.map((title, i) => ({
    id: `demo-thread-${i}`,
    title,
    cwd: `/demo/${['codex-atelier', 'design-system', 'research'][i % 3]}`,
    provider: i === 0 || i === 10 ? 'legacy-relay' : 'atelier',
    model: i % 3 === 0 ? 'gpt-5.4' : 'gpt-5.3-codex',
    updatedAt: now - i * 3600_000 * 5,
    createdAt: now - i * 86400_000,
    archived: i === 7 || i === 10,
    tokens: i === 7 ? null : [4850000, 805900, 5600000, 256600, 1100000][i % 5]!,
    rolloutPath: `/demo/sessions/${i}.jsonl`,
    source: 'desktop',
  }))
  const plans = new Map<string, ThreadPlan>()
  const history: { backup: ThreadBackup; old: ThreadRecord[]; after: ThreadRecord[] }[] = []
  function plan(ids: string[], action: ThreadPlan['action'], value: string) {
    const p: ThreadPlan = {
      ticket: crypto.randomUUID(),
      action,
      changes: rows
        .filter((r) => ids.includes(r.id))
        .map((r) => ({
          id: r.id,
          title: r.title,
          before: action === 'provider' || action === 'migration' ? r.provider : String(r.archived),
          after: value,
        }))
        .filter((c) => c.before !== c.after),
      warnings: [
        '演示模式：只修改内存中的示例数据，不影响本机 Codex。',
        ...(action === 'provider' || action === 'migration'
          ? ['此操作只对齐索引中的 Provider；不会转换加密历史或改写会话文件。']
          : []),
      ],
    }
    plans.set(p.ticket, p)
    return p
  }
  return {
    async catalog() {
      return {
        source: 'demo',
        database: '演示索引 · 不连接本机',
        provider: 'atelier',
        checkedAt: Date.now(),
        threads: structuredClone(rows),
        warnings: [],
      }
    },
    async detail(id) {
      const r = rows.find((r) => r.id === id)!
      const total = r.tokens
      const d: ThreadDetail = {
        id,
        bytes: total ? Math.floor(total * 1.6) : 0,
        turns: 28,
        usage: total
          ? {
              input: Math.floor(total * 0.94),
              cached: Math.floor(total * 0.91),
              output: Math.floor(total * 0.06),
              reasoning: Math.floor(total * 0.02),
              total,
            }
          : null,
        tools: [
          { name: 'exec_command', count: 16 },
          { name: 'apply_patch', count: 8 },
          { name: 'view_image', count: 3 },
        ],
        timeline: total
          ? [
              { date: '2026-11-01', total: Math.floor(total * 0.12) },
              { date: '2026-11-03', total: Math.floor(total * 0.23) },
              { date: '2026-11-05', total },
            ]
          : [],
        modelUsage: total ? [{ model: r.model, total }] : [],
        models: [r.model],
        messages: [
          { role: 'user', text: '检查线程索引与当前 Provider 是否一致。' },
          {
            role: 'assistant',
            text: '已发现旧 Provider 记录。先预览差异，再确认修改；历史正文保持原样。',
          },
        ],
        warnings: total ? [] : ['记录文件缺失，用量未知。'],
        encrypted: id === 'demo-thread-0',
        complete: !!total,
      }
      return d
    },
    async preview(ids, action, value) {
      return plan(ids, action, value)
    },
    async apply(ticket) {
      const p = plans.get(ticket)
      if (!p) throw Error('预览已失效')
      const old = structuredClone(rows)
      for (const c of p.changes) {
        const r = rows.find((r) => r.id === c.id)!
        if (p.action === 'provider' || p.action === 'migration') r.provider = c.after
        else if (p.action === 'restore') {
          const entry = history.find((h) => h.backup.id === c.after)
          const prev = entry?.old.find((r) => r.id === c.id)
          if (prev) Object.assign(r, prev)
        } else r.archived = c.after === 'true'
      }
      history.push({
        backup: { id: ticket, createdAt: Date.now(), action: p.action, count: p.changes.length },
        old,
        after: structuredClone(rows),
      })
      plans.delete(ticket)
    },
    async backups() {
      return history.map((h) => h.backup).reverse()
    },
    async restore(id) {
      const h = history.find((h) => h.backup.id === id)
      if (!h) throw Error('备份不存在')
      const p: ThreadPlan = {
        ticket: crypto.randomUUID(),
        action: 'restore',
        changes: h.old
          .filter((r) => JSON.stringify(r) !== JSON.stringify(h.after.find((x) => x.id === r.id)))
          .map((r) => ({ id: r.id, title: r.title, before: '当前演示记录', after: id })),
        warnings: ['将恢复本次操作前的演示记录。'],
      }
      plans.set(p.ticket, p)
      return p
    },
  }
}
