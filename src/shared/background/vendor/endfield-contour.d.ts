export interface ContourOptions {
  enabled: boolean
  animated: boolean
  trail: boolean
}
export interface ContourRenderer {
  refresh(): void
  destroy(): void
}
export function mountEndfieldContour(
  host: HTMLElement,
  readOptions: () => ContourOptions,
): ContourRenderer
