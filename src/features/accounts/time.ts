export function fullDate(value: number | null | undefined) {
  if (value == null || !Number.isFinite(value) || Number.isNaN(new Date(value).getTime()))
    return '未知'
  return new Date(value).toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
  })
}
export function countdown(value: number | null | undefined, now: number) {
  if (value == null) return '时间未知'
  if (value <= now) return '已到时间，请刷新确认'
  const minutes = Math.ceil((value - now) / 60_000)
  const days = Math.floor(minutes / 1440)
  const hours = Math.floor((minutes % 1440) / 60)
  const mins = minutes % 60
  return (
    [days ? days + '天' : '', hours ? hours + '小时' : '', mins ? mins + '分钟' : '']
      .filter(Boolean)
      .join(' ') || '不足1分钟'
  )
}
