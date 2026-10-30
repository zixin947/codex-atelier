export type PonytailMode = 'lite' | 'full' | 'ultra' | 'off'
export type StsTaskMode = 'unconfirmed' | 'answer' | 'review' | 'monitor' | 'change' | 'open'
export type StsControlLevel = 'watch' | 'guard' | 'lock' | 'off'
export interface PonytailModeState {
  mode: PonytailMode
  configMode: PonytailMode | null
  environmentMode: PonytailMode | null
  revision: string
  path: string
  canRollback: boolean
  receipt: string | null
}
export interface StsSessionMode {
  id: string
  title: string
  cwd: string
  updatedAt: number | null
  mode: StsTaskMode
  level: StsControlLevel
  allowedPaths: string[] | null
  revision: string
  canRollback: boolean
  source: string
  receipt: string | null
}
export interface StsModeChange {
  sessionId: string
  expectedRevision: string
  mode: StsTaskMode
  level: StsControlLevel
  allowedPaths: string[] | null
}
export interface PluginModeService {
  ponytail(): Promise<PonytailModeState>
  setPonytail(mode: PonytailMode, expectedRevision: string): Promise<PonytailModeState>
  rollbackPonytail(expectedRevision: string): Promise<PonytailModeState>
  stsSessions(): Promise<{ sessions: StsSessionMode[]; skipped: number }>
  stsSession(sessionId: string): Promise<StsSessionMode>
  setSts(change: StsModeChange): Promise<StsSessionMode>
  rollbackSts(sessionId: string, expectedRevision: string): Promise<StsSessionMode>
}
