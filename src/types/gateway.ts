export type ProfileKind = 'chatgpt' | 'apiKey'
export interface GatewayModel {
  id: string
  name: string
}
export interface GatewayProfile {
  id: string
  kind: ProfileKind
  name: string
  email: string | null
  enabled: boolean
  priority: number
  baseUrl: string
  authStatus: 'ready' | 'needsLogin' | 'planDisabled' | 'unknown'
  models: GatewayModel[]
  cooldownUntil: number | null
  lastError: string | null
  requests: number
  inputTokens: number
  outputTokens: number
  createdAt: number
}
export interface GatewaySettings {
  port: number
  autoSwitch: boolean
  maxAttempts: number
  preferredProfileId: string | null
  model: string
}
export interface GatewayEvent {
  at: number
  profileId: string | null
  status: string
  code: string
  attempts: number
}
export interface GatewayRuntime {
  state: 'stopped' | 'running' | 'stopping' | 'error'
  address: string | null
  activeRequests: number
  completedRequests: number
  failedRequests: number
  message: string | null
}
export interface GatewaySnapshot {
  available: boolean
  reason: string | null
  revision: string
  profiles: GatewayProfile[]
  settings: GatewaySettings
  runtime: GatewayRuntime
  events: GatewayEvent[]
}
export interface ProviderInput {
  id?: string | null
  name: string
  baseUrl: string
  apiKey: string | null
  models: string[]
  priority: number
}
export interface ProfileUpdate {
  id: string
  name: string
  enabled: boolean
  priority: number
}
export interface LoginAttempt {
  id: string
  state: 'pending' | 'completed' | 'error' | 'cancelled'
  authUrl: string | null
  expiresAt: number
  message: string | null
}
export interface GatewayService {
  snapshot(): Promise<GatewaySnapshot>
  saveProvider(input: ProviderInput, revision: string): Promise<GatewaySnapshot>
  updateProfile(input: ProfileUpdate, revision: string): Promise<GatewaySnapshot>
  removeProfile(id: string, revision: string): Promise<GatewaySnapshot>
  saveSettings(settings: GatewaySettings, revision: string): Promise<GatewaySnapshot>
  selectProfile(id: string, revision: string): Promise<GatewaySnapshot>
  start(): Promise<GatewaySnapshot>
  stop(): Promise<GatewaySnapshot>
  localKey(): Promise<string>
  refreshModels(id: string): Promise<GatewaySnapshot>
  beginLogin(name: string, accountId?: string): Promise<LoginAttempt>
  openLogin(id: string): Promise<void>
  pollLogin(id: string): Promise<LoginAttempt>
  cancelLogin(id: string): Promise<void>
  signOut(id: string): Promise<GatewaySnapshot>
  preview(): Promise<string>
  launchDesktop(): Promise<string>
}
