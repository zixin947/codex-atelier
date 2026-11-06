import type { DiagnosticGroup, DiagnosticItem } from '../types/maintenance'

const definitions: [string, DiagnosticGroup, string][] = [
  ['config_access', 'config', 'config.toml 可以正常读取'],
  ['config_replace_readiness', 'config', '配置文件权限与类型'],
  ['config_toml_syntax', 'config', 'TOML 语法'],
  ['config_third_party', 'config', '第三方注入残留'],
  ['config_profile_conflict', 'config', '顶层 profile 冲突'],
  ['config_stale', 'config', '当前连接配置一致性'],
  ['config_global_validity', 'config', '全局配置引用'],
  ['router_recovery_record', 'config', '连接恢复记录'],
  ['router_http_contract', 'config', 'Router HTTP 契约'],
  ['router_postcondition', 'config', '连接目标状态'],
  ['takeover_backup_orphan', 'config', '接管备份残留'],
  ['catalog_path_validity', 'config', '模型目录路径'],
  ['catalog_integrity', 'config', '模型目录完整性'],
  ['official_catalog_freshness', 'config', '官方模型目录新鲜度'],
  ['auth_integrity', 'auth', '登录文件完整性'],
  ['router_unlock_auth_residue', 'auth', '连接认证残留'],
  ['api_key_integrity', 'auth', '中转凭据完整性'],
  ['auth_token_expiry', 'auth', '登录凭据有效期'],
  ['proxy_running', 'runtime', '本地代理运行状态'],
  ['codex_process_state', 'runtime', 'Desktop 进程状态'],
  ['db_orphan_providers', 'runtime', '数据库 Provider 引用'],
  ['rollout_orphan_providers', 'runtime', '会话记录 Provider 引用'],
  ['thread_missing_rollouts', 'runtime', '会话记录文件'],
  ['thread_history_integrity', 'runtime', '线程历史完整性'],
  ['thread_provider_consistency', 'runtime', '线程 Provider 一致性'],
  ['thread_model_catalog_consistency', 'runtime', '线程模型与目录一致性'],
  ['native_off_invalid_openai_thread_models', 'runtime', '官方连接下的历史模型'],
  ['thread_poisoned_instructions', 'runtime', '线程指令完整性'],
  ['thread_response_item_ids', 'runtime', '历史工具调用标识'],
  ['legacy_migrated_threads', 'runtime', '旧版迁移线程'],
  ['residual_manifest', 'runtime', '迁移清单残留'],
  ['residual_cache', 'runtime', '连接缓存引用'],
  ['codex_home_writable', 'env', '数据目录权限'],
  ['project_state_integrity', 'env', '项目状态完整性'],
  ['state_db_relocation', 'env', '线程数据库位置'],
]
export function createDiagnosticDemo(): DiagnosticItem[] {
  return definitions.map(([id, group, label]) => {
    const problem = id === 'config_profile_conflict' || id === 'thread_provider_consistency'
    const skipped = [
      'router_http_contract',
      'api_key_integrity',
      'router_unlock_auth_residue',
    ].includes(id)
    const unknown = id === 'official_catalog_freshness'
    return {
      id,
      group,
      label,
      status: problem ? 'warning' : skipped ? 'skipped' : unknown ? 'unknown' : 'ok',
      fixable: problem,
      detail:
        id === 'config_profile_conflict'
          ? '演示检查：发现旧的顶层 profile 引用，可移除失效引用并保留 profiles 配置。'
          : id === 'thread_provider_consistency'
            ? '演示检查：2 个线程的索引与会话记录使用了不同的 Provider。修复前将备份并检查文件是否变化。'
            : skipped
              ? '演示环境未启用该连接方式，此项不适用。'
              : unknown
                ? '尚无官方目录刷新记录，无法确认新鲜度。'
                : '演示检查通过。浏览器未读取本机数据。',
    }
  })
}

// A real, uncompressed ZIP containing only the labelled demonstration document.
export function demoZipBytes(): number[] {
  const name = new TextEncoder().encode('demo-report.txt')
  const body = new TextEncoder().encode(
    'Codex Atelier — DEMO ONLY\nNo local files, credentials or conversations were read.\n',
  )
  let crc = 0xffffffff
  for (const b of body) {
    crc ^= b
    for (let bit = 0; bit < 8; bit++) crc = (crc >>> 1) ^ (crc & 1 ? 0xedb88320 : 0)
  }
  crc = (crc ^ 0xffffffff) >>> 0
  const bytes = new Uint8Array(30 + name.length + body.length + 46 + name.length + 22)
  const v = new DataView(bytes.buffer)
  const u16 = (at: number, n: number) => v.setUint16(at, n, true)
  const u32 = (at: number, n: number) => v.setUint32(at, n, true)
  u32(0, 0x04034b50)
  u16(4, 20)
  u32(14, crc)
  u32(18, body.length)
  u32(22, body.length)
  u16(26, name.length)
  bytes.set(name, 30)
  bytes.set(body, 30 + name.length)
  const cd = 30 + name.length + body.length
  u32(cd, 0x02014b50)
  u16(cd + 4, 20)
  u16(cd + 6, 20)
  u32(cd + 16, crc)
  u32(cd + 20, body.length)
  u32(cd + 24, body.length)
  u16(cd + 28, name.length)
  bytes.set(name, cd + 46)
  const end = cd + 46 + name.length
  u32(end, 0x06054b50)
  u16(end + 8, 1)
  u16(end + 10, 1)
  u32(end + 12, 46 + name.length)
  u32(end + 16, cd)
  return [...bytes]
}
