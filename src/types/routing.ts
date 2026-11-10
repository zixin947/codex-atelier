import type { GatewayRuntime } from './gateway'
export interface RouteCandidate {
  profileId: string
  name: string
  eligible: boolean
  reason: string
}
export interface RouteAttempt {
  profileId: string
  startedAt: number
  durationMs: number | null
  httpStatus: number | null
  code: string
}
export interface RouteTrace {
  id: string
  at: number
  model: string
  endpoint: string
  status: 'routing' | 'forwarding' | 'completed' | 'failed'
  code: string
  durationMs: number | null
  candidates: RouteCandidate[]
  attempts: RouteAttempt[]
}
export interface RoutingSnapshot {
  runtime: GatewayRuntime
  reroutedRequests: number
  routes: RouteTrace[]
}
