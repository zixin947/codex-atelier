import { invoke, isTauri } from '@tauri-apps/api/core'
import type { RoutingSnapshot } from '../types/routing'

export function readRouting(): Promise<RoutingSnapshot | null> {
  return isTauri() ? invoke('gateway_routing_snapshot') : Promise.resolve(null)
}
