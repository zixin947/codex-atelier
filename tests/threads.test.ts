import { describe, it, expect } from 'vitest'
import { createThreadDemo } from '../src/services/threadsMock'
import {
  filterThreads,
  usageSegments,
  exportMarkdown,
  scopeThreads,
} from '../src/features/threads/model'
describe('thread workspace', () => {
  it('combines filters without mutating input', async () => {
    const s = createThreadDemo()
    const c = await s.catalog()
    const r = filterThreads(c.threads, {
      search: '索引',
      project: '/demo/codex-atelier',
      archive: 'active',
      model: '',
      start: '',
      end: '',
      sort: 'updated',
    })
    expect(r).toHaveLength(1)
    expect(c.threads).toHaveLength(12)
  })
  it('unknown usage is not zero', async () => {
    const d = await createThreadDemo().detail('demo-thread-7')
    expect(d.usage).toBeNull()
    expect(d.complete).toBe(false)
  })
  it('deducts subsets rather than counting cached/reasoning twice', () => {
    expect(usageSegments({ input: 100, cached: 80, output: 20, reasoning: 5, total: 120 })).toEqual(
      [20, 80, 15, 5],
    )
  })
  it('preview is read only, apply changes only selected rows, rollback restores', async () => {
    const s = createThreadDemo()
    const p = await s.preview(['demo-thread-0'], 'provider', 'atelier')
    expect((await s.catalog()).threads[0]?.provider).toBe('legacy-relay')
    await s.apply(p.ticket)
    expect((await s.catalog()).threads[0]?.provider).toBe('atelier')
    expect((await s.catalog()).threads[10]?.provider).toBe('legacy-relay')
    const restore = await s.restore(p.ticket)
    await s.apply(restore.ticket)
    expect((await s.catalog()).threads[0]?.provider).toBe('legacy-relay')
  })
  it('exports plain text without interpreting message HTML', () => {
    expect(exportMarkdown('test', [{ role: 'user', text: '<script>hello</script>' }])).toContain(
      '<script>hello</script>',
    )
  })
})

describe('thread counting scope', () => {
  it('keeps missing-file conversations while separating subtasks and unverified rows', async () => {
    const base = (await createThreadDemo().catalog()).threads[0]!
    const rows = [
      { ...base, id: 'user', kind: 'conversation' as const, rolloutPath: 'missing.jsonl' },
      { ...base, id: 'agent', kind: 'subtask' as const },
      { ...base, id: 'old-index', kind: 'unverified' as const },
    ]
    expect(scopeThreads(rows, 'conversation').map((r) => r.id)).toEqual(['user'])
    expect(scopeThreads(rows, 'subtask').map((r) => r.id)).toEqual(['agent'])
    expect(scopeThreads(rows, 'unverified').map((r) => r.id)).toEqual(['old-index'])
    expect(scopeThreads(rows, 'all')).toHaveLength(3)
    expect(rows).toHaveLength(3)
  })
})
