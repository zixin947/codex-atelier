import { readFileSync } from 'node:fs'
import { describe, expect, it } from 'vitest'
const body = readFileSync('src/features/connections/native-routing-path.js', 'utf8')
const curve = new Function('location', 'x0', 'y0', 'mx', 'x3', 'y3', body)
describe('blueprint measured conveyor', () => {
  it('keeps measured endpoints for rising, falling and level paths', () => {
    for (const y of [20, 90, 160]) {
      const path = curve({ search: '?atelierStyle=endfield' }, 100, 90, 150, 200, y)
      expect(path).toMatch(/^M100 90 L/)
      expect(path).toContain(`L200 ${y}`)
      expect(path).not.toMatch(/NaN|Infinity/)
    }
  })
  it('leaves other skins on the upstream path', () => {
    expect(curve({ search: '' }, 100, 90, 150, 200, 160)).toBeUndefined()
  })
})
