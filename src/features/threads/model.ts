import type { ThreadRecord, ThreadUsage } from '../../types/threads'
export interface ThreadFilter {
  search: string
  project: string
  archive: string
  model: string
  start: string
  end: string
  sort: string
}
export function filterThreads(rows: ThreadRecord[], f: ThreadFilter) {
  const query = f.search.trim().toLocaleLowerCase()
  const from = f.start ? new Date(`${f.start}T00:00:00`).getTime() : -Infinity
  const to = f.end ? new Date(`${f.end}T23:59:59.999`).getTime() : Infinity
  return rows
    .filter(
      (r) =>
        (!query ||
          [r.title, r.cwd, r.model, r.provider, r.id]
            .join(' ')
            .toLocaleLowerCase()
            .includes(query)) &&
        (!f.project || r.cwd === f.project) &&
        (f.archive === 'all' || r.archived === (f.archive === 'archived')) &&
        (!f.model || r.model === f.model) &&
        r.updatedAt >= from &&
        r.updatedAt <= to,
    )
    .sort((a, b) =>
      f.sort === 'tokens'
        ? (b.tokens ?? -1) - (a.tokens ?? -1)
        : f.sort === 'title'
          ? a.title.localeCompare(b.title)
          : b.updatedAt - a.updatedAt,
    )
}
export function compact(n: number | null): string {
  if (n === null) return '—'
  return Intl.NumberFormat('en', { notation: 'compact', maximumFractionDigits: 1 }).format(n)
}
export function projectName(path: string) {
  return path.split(/[\\/]/).filter(Boolean).pop() || '未关联项目'
}
export function usageSegments(u: ThreadUsage) {
  return [
    Math.max(0, u.input - u.cached),
    u.cached,
    Math.max(0, u.output - u.reasoning),
    u.reasoning,
  ]
}
export function exportMarkdown(title: string, messages: { role: string; text: string }[]) {
  return (
    `# ${title.replace(/[\r\n]/g, ' ')}\n\n` +
    messages.map((m) => `## ${m.role}\n\n${m.text}\n`).join('\n')
  )
}

export function threadKind(row: ThreadRecord): 'conversation' | 'subtask' | 'unverified' {
  if (row.kind) return row.kind
  if (row.source.toLowerCase().includes('subagent')) return 'subtask'
  return ['cli', 'vscode', 'desktop', 'app-server', 'app_server'].includes(row.source.toLowerCase())
    ? 'conversation'
    : 'unverified'
}
export function scopeThreads(rows: ThreadRecord[], scope: string) {
  return rows.filter(
    (row) =>
      scope === 'all' ||
      (scope === 'subtask'
        ? threadKind(row) === 'subtask'
        : scope === 'unverified'
          ? threadKind(row) === 'unverified'
          : threadKind(row) === 'conversation'),
  )
}
