// @vitest-environment node
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import crypto from 'node:crypto'
import { afterEach, expect, it, vi } from 'vitest'
import { createInsightsService } from '../vendor/gpt-dragon-girl/runtime/insights.mjs'

let directory: string
let service: ReturnType<typeof createInsightsService>
afterEach(() => {
  service?.close()
  if (directory) fs.rmSync(directory, { recursive: true, force: true })
  vi.unstubAllEnvs()
})

it.each([
  ['当前账号双窗口', false, false, true],
  ['切换账号后拒绝旧额度', true, false, false],
  ['过期结果明确标记', false, true, true],
])('%s', async (_label, changedAccount, expired, available) => {
  directory = fs.mkdtempSync(path.join(os.tmpdir(), 'atelier-quota-test-'))
  const auth = JSON.stringify({ auth_mode: 'chatgpt', tokens: { access_token: 'fixture' } })
  fs.writeFileSync(path.join(directory, 'auth.json'), auth)
  const now = Date.now()
  const observedAt = expired ? now - 1_800_000 : now
  const file = path.join(directory, 'quota.json')
  vi.stubEnv('ATELIER_QUOTA_FILE', file)
  fs.writeFileSync(
    file,
    JSON.stringify({
      version: 1,
      authFingerprint: changedAccount
        ? 'different'
        : crypto.createHash('sha256').update(auth).digest('hex'),
      subscription: {
        observedAt,
        reason: 'Atelier',
        windows: [300, 10080].map((windowDurationMins) => ({
          label: 'fixture',
          usedPercent: 10,
          windowDurationMins,
          resetsAt: now + 3_600_000,
          observedAt,
          stale: false,
        })),
      },
    }),
  )
  service = createInsightsService({
    codexHome: directory,
    resolve: () => ({
      id: 'custom-gateway',
      baseUrl: 'http://127.0.0.1:7777',
      key: '',
      accountId: 'fixture',
      setting: { monitorSessions: true },
    }),
  })
  const result = await service.get()
  expect(result.subscription.available).toBe(available)
  if (available) {
    expect(result.subscription.source).toBe('atelier')
    expect(result.subscription.windows).toHaveLength(2)
    expect(result.subscription.windows[0].stale).toBe(expired)
  }
})
