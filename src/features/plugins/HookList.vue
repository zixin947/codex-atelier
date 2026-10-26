<script setup lang="ts">
import type { HookDetail } from '../../types/plugins'
defineProps<{ pluginName: string; pluginId: string; hooks: HookDetail[] }>()
const events: Record<string, string> = {
  sessionStart: '会话启动时',
  userPromptSubmit: '提交消息时',
  subagentStart: '子代理启动时',
  subagentStop: '子代理结束时',
  sessionEnd: '会话结束时',
  preToolUse: '调用工具前',
  postToolUse: '调用工具后',
  permissionRequest: '请求权限时',
  preCompact: '压缩上下文前',
  postCompact: '压缩上下文后',
  preModelRequest: '请求模型前',
  postModelRequest: '请求模型后',
  stop: '本轮结束时',
  interrupt: '中断任务时',
}
const states = {
  trusted: '已信任',
  managed: '策略信任',
  untrusted: '待信任',
  modified: '定义已变化',
}
function eventLabel(event: string) {
  return events[event] ?? '其他触发时机'
}
function eventCode(event: string) {
  return events[event] ? event[0]!.toUpperCase() + event.slice(1) : event
}
</script>
<template>
  <section v-if="hooks.length" class="hook-list" :aria-label="`${pluginName} Hook 明细`">
    <p class="hook-list-intro">在 CLI /hooks 中，按插件来源和下列触发事件核对。</p>
    <ol>
      <li v-for="(hook, index) in hooks" :key="hook.key ?? index" class="hook-row">
        <div class="hook-row-heading">
          <span class="hook-number mono">{{ String(index + 1).padStart(2, '0') }}</span>
          <div>
            <span class="hook-owner">{{ pluginName }}</span>
            <h3>{{ eventLabel(hook.eventName) }}</h3>
          </div>
          <span
            class="hook-state"
            :class="{ review: hook.trustStatus === 'untrusted' || hook.trustStatus === 'modified' }"
            >{{ states[hook.trustStatus] }}</span
          >
        </div>
        <div class="hook-event">
          <code>{{ eventCode(hook.eventName) }}</code
          ><span v-if="!hook.enabled">此 Hook 已单独停用</span>
        </div>
        <details class="hook-details">
          <summary>核对命令与来源</summary>
          <dl>
            <dt>所属插件</dt>
            <dd>
              <code>{{ pluginId }}</code>
            </dd>
            <dt>配置来源</dt>
            <dd>
              <code>{{ hook.sourcePath ?? 'CLI 未返回来源路径' }}</code>
            </dd>
            <dt>处理类型</dt>
            <dd>{{ hook.handlerType }}</dd>
            <template v-if="hook.command"
              ><dt>实际命令</dt>
              <dd>
                <code class="hook-command">{{ hook.command }}</code>
              </dd></template
            >
            <template v-if="hook.mcpTarget"
              ><dt>MCP 工具</dt>
              <dd>
                <code>{{ hook.mcpTarget }}</code>
              </dd></template
            >
            <template v-if="hook.matcher"
              ><dt>匹配条件</dt>
              <dd>
                <code>{{ hook.matcher }}</code>
              </dd></template
            >
            <template v-if="hook.key"
              ><dt>Hook 标识</dt>
              <dd>
                <code>{{ hook.key }}</code>
              </dd></template
            >
          </dl>
          <p>仅展示宿主返回的信息；请核对相同来源、事件和命令后再决定是否信任。</p>
        </details>
      </li>
    </ol>
  </section>
</template>
