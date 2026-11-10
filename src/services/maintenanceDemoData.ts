import type { DiagnosticGroup, DiagnosticItem } from '../types/maintenance'

/* 演示数据。条目 id、分组与 label 对齐 AiMaMi 1.2.7 系统诊断：
   分组表来自原版前端常量（config 12 / auth 4 / runtime 12 / env 1），
   未在表内的 6 项按原版逻辑回退到 runtime。 */

const definitions: [string, DiagnosticGroup, string][] = [
  ['config_toml_syntax', 'config', 'TOML 语法正常'],
  ['config_third_party', 'config', '无第三方注入残留'],
  ['config_profile_conflict', 'config', '顶层 profile 冲突'],
  ['config_stale', 'config', 'Codex 配置一致性正常'],
  ['router_http_contract', 'config', 'Router HTTP 契约（路由未启用，跳过）'],
  ['takeover_backup_orphan', 'config', '无 takeover 备份残留'],
  ['catalog_path_validity', 'config', 'Catalog 路径有效'],
  ['catalog_integrity', 'config', 'Catalog 完整'],
  ['official_catalog_freshness', 'config', '尚未观测到目录请求，不能据此确认官方目录可用'],
  ['config_global_validity', 'config', 'Codex 配置一致性正常'],
  ['config_access', 'config', 'config.toml 可正常读取'],
  ['config_replace_readiness', 'config', 'config.toml 权限未显示只读'],
  ['auth_integrity', 'auth', 'auth.json 正常'],
  ['api_key_integrity', 'auth', 'API 密钥完整'],
  ['auth_token_expiry', 'auth', 'Token 过期检测（API Key 模式，不适用）'],
  ['router_unlock_auth_residue', 'auth', '无虚拟账号残留'],
  ['proxy_running', 'runtime', '本地代理运行中'],
  ['thread_missing_rollouts', 'runtime', '无示例线程 rollout 文件完整'],
  ['thread_history_integrity', 'runtime', '线程历史结构自洽'],
  ['thread_provider_consistency', 'runtime', '未发现失效的旧版工具线程 provider'],
  ['thread_poisoned_instructions', 'runtime', '线程调用项 ID 健康（已检查 12 个）'],
  ['thread_response_item_ids', 'runtime', '线程调用项 ID 健康（已检查 12 个）'],
  ['db_orphan_providers', 'runtime', '线程数据（数据库不存在，跳过）'],
  ['rollout_orphan_providers', 'runtime', '无第三方注入残留'],
  ['residual_manifest', 'runtime', '无残留迁移 manifest'],
  ['residual_cache', 'runtime', '无残留 relay-state-cache'],
  ['codex_process_state', 'runtime', 'Codex 运行中'],
  ['state_db_relocation', 'runtime', '活库不在 sqlite/ 子目录，无需清理'],
  ['router_postcondition', 'runtime', '目标态非配置维度稳定（配置偏差由 config_stale 报告）'],
  ['thread_model_catalog_consistency', 'runtime', 'Catalog 与卡片模型及历史兼容映射一致'],
  ['legacy_migrated_threads', 'runtime', '无旧版线程迁移残留'],
  ['project_state_integrity', 'runtime', 'Codex 项目索引正常'],
  ['native_off_invalid_openai_thread_models', 'runtime', 'NativeOff 官方线程模型正常'],
  ['router_recovery_record', 'runtime', '路由恢复记录可正常读取'],
  ['codex_home_writable', 'env', '文件系统正常（剩余 82 GB）'],
]

export function createDiagnosticDemo(): DiagnosticItem[] {
  return definitions.map(([id, group, label]) => {
    const problem = id === 'config_profile_conflict' || id === 'thread_provider_consistency'
    // 原版没有 skipped / unknown：跳过检测走 ok，未确认走 warning。
    const skipped = [
      'router_http_contract',
      'api_key_integrity',
      'router_unlock_auth_residue',
    ].includes(id)
    const unknown = id === 'official_catalog_freshness'
    return {
      id,
      group,
      label: problem
        ? id === 'config_profile_conflict'
          ? '无 legacy 顶层 profile'
          : '1 条主线程引用了失效的旧版工具 provider'
        : label,
      status: problem || unknown ? 'warning' : 'ok',
      fixable: problem,
      detail:
        id === 'config_profile_conflict'
          ? 'Codex 0.134+ 已不再支持 config.toml 顶层 profile；智能路由应改用 model_provider + model_catalog_json 的桌面端路由契约。顶层 profile 配置会被清理，profiles 名称保留。这是演示数据。'
          : id === 'thread_provider_consistency'
            ? '这些主线程在 Codex 中可能无法正常对话。修复会同步接管 DB 与 Rollout，保留仍有效的官方模型，否则回落当前官方默认模型。仅修复旧版工具残留；混合外部归属时停止，未确认的第三方任务不接管。示例：演示线程。'
            : skipped
              ? '当前模式不需要处理这一项。'
              : unknown
                ? '等待 ChatGPT 原生目录请求；已有有效目录由 ChatGPT 原生管理，无可用目录时不保证混合菜单。'
                : '浏览器未读取本机数据，这一项是演示结果。',
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
  const crcBytes = [24, 16, 8, 0].map((s) => (crc >>> s) & 0xff)
  const u16 = (n: number) => [n & 0xff, (n >> 8) & 0xff]
  const u32 = (n: number) => [n & 0xff, (n >>> 8) & 0xff, (n >>> 16) & 0xff, (n >>> 24) & 0xff]
  const local = [
    0x50,
    0x4b,
    0x03,
    0x04,
    ...u16(20),
    ...u16(0),
    ...u16(0),
    ...u16(0),
    ...u16(0),
    ...crcBytes,
    ...u32(body.length),
    ...u32(body.length),
    ...u16(name.length),
    ...u16(0),
    ...name,
    ...body,
  ]
  const central = [
    0x50,
    0x4b,
    0x01,
    0x02,
    ...u16(20),
    ...u16(20),
    ...u16(0),
    ...u16(0),
    ...u16(0),
    ...u16(0),
    ...crcBytes,
    ...u32(body.length),
    ...u32(body.length),
    ...u16(name.length),
    ...u16(0),
    ...u16(0),
    ...u16(0),
    ...u16(0),
    ...u32(0),
    ...u32(0),
    ...name,
  ]
  const end = [
    0x50,
    0x4b,
    0x05,
    0x06,
    ...u16(0),
    ...u16(0),
    ...u16(1),
    ...u16(1),
    ...u32(central.length),
    ...u32(local.length),
    ...u16(0),
  ]
  return [...local, ...central, ...end]
}
