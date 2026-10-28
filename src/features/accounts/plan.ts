import type { CodexAccount } from '../../types/accounts'

type BadgeTone = 'unknown' | 'free' | 'plus' | 'pro100' | 'pro200' | 'pro500' | 'organization'
interface PlanBadge {
  label: string
  tone: BadgeTone
  title: string
}

const proTiers: Record<string, 100 | 200 | 500> = {
  prolite: 100,
  'pro-lite': 100,
  'pro-5x': 100,
  'codex-pro-5x': 100,
  'pro-100': 100,
  chatgptprolite: 100,
  pro: 200,
  'pro-20x': 200,
  'codex-pro-20x': 200,
  'pro-200': 200,
  chatgptpro: 200,
  promax: 500,
  'pro-max': 500,
  'pro-500': 500,
  chatgptpromax: 500,
}
const planNames: Record<string, { label: string; tone: BadgeTone }> = {
  free: { label: 'Free', tone: 'free' },
  go: { label: 'Go', tone: 'free' },
  plus: { label: 'Plus', tone: 'plus' },
  team: { label: 'Team', tone: 'organization' },
  business: { label: 'Business', tone: 'organization' },
  enterprise: { label: 'Enterprise', tone: 'organization' },
  edu: { label: 'Edu', tone: 'organization' },
}

/** Translate a plan identifier; the displayed dollar tier is not a receipt or currency conversion. */
export function accountPlanBadge(account: CodexAccount): PlanBadge {
  const raw = account.plan?.trim() || account.metadata.planSnapshot?.trim() || ''
  if (!raw) return { label: '未知套餐', tone: 'unknown', title: '尚未取得套餐标识' }
  const key = raw.toLowerCase().replace(/[_\s]+/g, '-')
  const tier = Object.hasOwn(proTiers, key) ? proTiers[key] : undefined
  if (tier) {
    return {
      label: 'Pro $' + tier,
      tone: ('pro' + tier) as BadgeTone,
      title: 'ChatGPT Pro $' + tier + ' 档位 · 套餐标识：' + raw + '；不是实际扣费金额',
    }
  }
  const nameKey = Object.hasOwn(planNames, key) ? key : key.replace(/^chatgpt-?/, '')
  const named = Object.hasOwn(planNames, nameKey) ? planNames[nameKey] : undefined
  return named
    ? { ...named, title: 'ChatGPT ' + named.label + ' · 套餐标识：' + raw }
    : { label: raw, tone: 'unknown', title: '尚未识别的新套餐标识：' + raw }
}
