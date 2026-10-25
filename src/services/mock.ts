import type { AtelierService, Extension, Check } from '../types/atelier'
const examples: Extension[] = [
  {
    id: 'filesystem',
    name: 'Filesystem',
    subtitle: '工作区文件访问',
    kind: 'MCP',
    enabled: true,
    version: '2026.1',
    description: '在指定工作区中查找、读取和整理文件。仅展示示例权限，不连接本机文件系统。',
    tools: ['read_file', 'list_directory', 'search_files'],
  },
  {
    id: 'github',
    name: 'GitHub',
    subtitle: '仓库与协作',
    kind: 'MCP',
    enabled: true,
    version: '0.32',
    description: '查看代码仓库、Issue 和 Pull Request。本原型没有登录、令牌或网络调用。',
    tools: ['search_repositories', 'get_pull_request'],
  },
  {
    id: 'context7',
    name: 'Context7',
    subtitle: '开发文档检索',
    kind: 'MCP',
    enabled: true,
    version: '1.0',
    description: '为开发任务提供库文档上下文。连接状态和工具均为示例。',
    tools: ['resolve_library', 'query_docs'],
  },
  {
    id: 'browser',
    name: 'Browser tools',
    subtitle: '浏览器自动化',
    kind: 'MCP',
    enabled: false,
    version: '0.8',
    description: '页面检查与交互的示例扩展。开关只改变内存状态。',
    tools: ['navigate', 'screenshot'],
  },
  {
    id: 'review',
    name: 'Code review',
    subtitle: '有依据的代码审阅',
    kind: 'Skills',
    enabled: true,
    version: '1.2',
    description: '结构化审查代码变更，记录风险和验证边界。此处不执行真实审阅。',
    tools: ['review_workflow'],
  },
  {
    id: 'design',
    name: 'Interface design',
    subtitle: '界面设计与验收',
    kind: 'Skills',
    enabled: true,
    version: '1.0',
    description: '从视觉规范到交互验收的工作流示例。',
    tools: ['design_workflow'],
  },
  {
    id: 'docs',
    name: 'Documentation',
    subtitle: '项目文档维护',
    kind: 'Skills',
    enabled: false,
    version: '1.1',
    description: '整理和维护文档的技能示例，不执行真实文件操作。',
    tools: ['documentation_workflow'],
  },
]
const checks: Check[] = [
  {
    id: 'runtime',
    title: '应用运行环境',
    group: '运行环境',
    status: 'pass',
    detail: '示例环境中的 CLI、运行时与工作目录均可用。未检测本机 Codex。',
    risk: '只读检查的演示结果，不代表本机状态。',
  },
  {
    id: 'provider',
    title: 'Provider 配置重复',
    group: '配置一致性',
    status: 'warning',
    detail: '示例配置中存在两条同名 Provider，可能造成路由歧义。建议合并重复项。',
    risk: '真实修复可能改变模型路由；未来需要检测外部变更、备份并明确确认。',
  },
  {
    id: 'extension',
    title: '扩展路径已迁移',
    group: '扩展配置',
    status: 'warning',
    detail: '示例 Browser tools 指向旧路径，建议更新演示路径。',
    risk: '真实修改可能影响扩展加载。本轮只模拟差异。',
  },
  {
    id: 'backup',
    title: '备份完整性',
    group: '备份与恢复',
    status: 'pass',
    detail: '3 份示例备份的校验状态正常，未扫描或创建本机备份。',
    risk: '恢复真实备份会覆盖配置，本轮不具备该能力。',
  },
]
const delay = () => new Promise<void>((r) => setTimeout(r, 280))
export function createMockAdapter(): AtelierService {
  const data = structuredClone(examples)
  return {
    async listBackups() {
      return [
        { id: 'b1', name: '工作配置 · 修改前', date: '2026-10-25 14:32', size: '4.8 KB' },
        { id: 'b2', name: '默认配置 · 手动备份', date: '2026-10-24 09:15', size: '4.6 KB' },
        { id: 'b3', name: '初始配置', date: '2026-10-23 18:06', size: '3.9 KB' },
      ]
    },
    async listExtensions() {
      await delay()
      return structuredClone(data)
    },
    async toggle(id) {
      await delay()
      const e = data.find((e) => e.id === id)
      if (!e) throw Error('示例扩展不存在')
      e.enabled = !e.enabled
      return structuredClone(e)
    },
    async diagnose() {
      await delay()
      return structuredClone(checks)
    },
    async preview(id) {
      await delay()
      return {
        id,
        revision: 'demo-01',
        before: id.startsWith('b')
          ? 'profile = "current-demo"'
          : id === 'extension'
            ? 'path = "/demo/old/browser"'
            : 'provider = "atelier-demo"\nprovider_alias = "atelier-demo"',
        after: id.startsWith('b')
          ? 'profile = "backup-demo"'
          : id === 'extension'
            ? 'path = "/demo/workspace/browser"'
            : 'provider = "atelier-demo"\n# duplicate alias removed',
      }
    },
    async confirm() {
      await delay()
    },
  }
}
export const service = createMockAdapter()
