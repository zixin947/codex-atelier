<script setup lang="ts">
import { computed, ref } from 'vue'
import { useAtelier } from '../app/store'
import Panel from '../shared/components/Panel.vue'
import Icon from '../shared/components/Icon.vue'
const store = useAtelier(),
  scanning = ref(false)
const enabled = computed(() => store.extensions.filter((e) => e.enabled).length)
async function scan() {
  scanning.value = true
  await store.load()
  scanning.value = false
  store.notify('演示检查完成 · 未扫描本机环境')
}
</script>
<template>
  <div class="page dashboard">
    <div class="page-heading">
      <div>
        <p class="eyebrow">WORKSPACE / OVERVIEW</p>
        <h1>一切就绪，开始创作<span class="heading-dot">.</span></h1>
        <p class="subtitle">把复杂留给工作台，把专注留给你。</p>
      </div>
      <div class="edition">
        <span>PERSONAL WORKSTATION</span><b>No. 001 — {{ store.profile }}</b>
      </div>
    </div>
    <div class="overview-layout">
      <div class="overview-main">
        <section class="instrument">
          <div class="instrument-header">
            <span><i class="dot" /> SYSTEM OVERVIEW</span><span>SIMULATED SIGNAL · 01</span>
          </div>
          <div class="instrument-core">
            <div class="instrument-copy">
              <span class="screen-label">CODEX ENVIRONMENT / 示例环境</span>
              <h2>运行状态良好<span class="cursor-block" /></h2>
              <p>工作配置已就位，随时进入心流。</p>
              <div class="screen-tags">
                <span><Icon name="check" :size="13" /> CLI 就绪</span
                ><span><Icon name="check" :size="13" /> 连接正常</span><span>DEMO</span>
              </div>
            </div>
            <div class="signal-art" aria-hidden="true">
              <div class="orbit one" />
              <div class="orbit two" />
              <div class="signal-chip"><Icon name="chip" :size="43" /></div>
              <span class="coordinate">X 024 / Y 128</span><i class="signal-point" />
            </div>
          </div>
          <div class="instrument-stats">
            <div>
              <small>当前模型 · 示例</small><b>GPT-5.4 <span>CODEX</span></b>
            </div>
            <div>
              <small>连接方式</small><b>Provider API <i class="dot" /></b>
            </div>
            <div>
              <small>扩展已启用</small
              ><b
                >{{ String(enabled).padStart(2, '0') }}
                <em>/ {{ String(store.extensions.length).padStart(2, '0') }}</em></b
              >
            </div>
          </div>
          <div class="instrument-bottom">
            <span>所有状态均为示例，未进行本机扫描</span><span class="mono">● MOCK / ONLINE</span>
          </div>
        </section>
        <div class="dashboard-pair">
          <Panel title="连接档案" code="CONNECTION"
            ><div class="connection-name">
              <span class="square-icon"><Icon name="plug" /></span>
              <div><b>Atelier Demo</b><small>标准 API · 示例 Provider</small></div>
              <span class="badge">正常</span>
            </div>
            <div class="inset definition">
              <span>端点</span><code>api.example.invalid/v1</code><span>工作配置</span
              ><b>{{ store.profile }}</b
              ><span>凭据状态</span><b>未配置 · 演示占位</b>
            </div>
            <RouterLink class="text-link" to="/planned/models"
              >查看模型与连接 <Icon name="arrow" :size="15" /></RouterLink></Panel
          ><Panel title="扩展工作区" code="EXTENSIONS"
            ><div class="extension-summary">
              <b>{{ String(enabled).padStart(2, '0') }}</b
              ><span>个扩展已启用<small>为你的工作流，添一双手。</small></span
              ><Icon name="box" :size="34" />
            </div>
            <div class="segment-meter" aria-label="扩展示例启用比例">
              <i v-for="n in 14" :key="n" :class="{ filled: n <= enabled * 2 }" />
            </div>
            <div class="split muted">
              <span
                >MCP 服务器
                <b
                  >{{ store.extensions.filter((e) => e.kind === 'MCP' && e.enabled).length }} / 4</b
                ></span
              ><span
                >Skills
                <b
                  >{{ store.extensions.filter((e) => e.kind === 'Skills' && e.enabled).length }} /
                  3</b
                ></span
              >
            </div>
            <RouterLink class="text-link" to="/extensions"
              >管理我的扩展 <Icon name="arrow" :size="15" /></RouterLink
          ></Panel>
        </div>
        <Panel title="最近活动" code="ACTIVITY LOG / 示例"
          ><div class="activity-row">
            <span class="activity-time">14:32</span
            ><span class="activity-icon"><Icon name="check" :size="14" /></span>
            <div><b>工作配置已载入</b><small>日常开发 · 演示配置</small></div>
            <span class="muted mono">CONFIG</span>
          </div>
          <div class="activity-row">
            <span class="activity-time">14:30</span
            ><span class="activity-icon orange-ink"><Icon name="pulse" :size="14" /></span>
            <div><b>示例诊断检查完成</b><small>2 项建议，等待你查看</small></div>
            <RouterLink class="text-link" to="/diagnostics"
              >查看 <Icon name="arrow" :size="14"
            /></RouterLink></div
        ></Panel>
      </div>
      <div class="overview-side">
        <Panel title="诊断摘要" code="02"
          ><div class="diagnostic-head">
            <span class="warning-symbol"><Icon name="alert" :size="25" /></span>
            <div><b>有 2 项值得留意</b><small>示例问题 · 不影响原型使用</small></div>
          </div>
          <RouterLink
            v-for="(title, i) in ['Provider 配置重复', '扩展路径已迁移']"
            :key="title"
            class="issue-row"
            to="/diagnostics"
            ><span class="issue-number">0{{ i + 1 }}</span>
            <div>
              <b>{{ title }}</b
              ><small>{{ i === 0 ? '建议合并重复的配置项' : '1 个扩展需要更新路径' }}</small>
            </div>
            <Icon name="chevron" :size="14" /></RouterLink
          ><button class="button full" :disabled="scanning" @click="scan">
            <Icon name="refresh" :class="{ spinning: scanning }" :size="16" />{{
              scanning ? '正在模拟检查…' : '重新检查（演示）'
            }}
          </button></Panel
        ><Panel title="快捷操作" code="QUICK ACCESS"
          ><RouterLink class="quick-action" to="/extensions"
            ><Icon name="box" /><span>管理 MCP 与 Skills</span
            ><Icon name="arrow" :size="15" /></RouterLink
          ><RouterLink class="quick-action" to="/diagnostics"
            ><Icon name="shield" /><span>查看备份与回滚</span
            ><Icon name="arrow" :size="15" /></RouterLink
          ><RouterLink class="quick-action" to="/planned/history"
            ><Icon name="history" /><span>浏览会话档案</span
            ><span class="tiny">规划中</span></RouterLink
          ></Panel
        >
        <div class="desk-note">
          <div class="note-label"><span>ATELIER NOTES</span><Icon name="grid" :size="13" /></div>
          <h3>好的工具，<br />让你忘记工具。</h3>
          <p>这里是 Codex 的外置工作间。<br />整理环境，然后安心创造。</p>
          <div class="note-bottom">
            <span class="mono">MAKE ROOM FOR IDEAS.</span
            ><span class="pixel-buddy" aria-label="桌宠展示占位">▟▙<br />▝▘</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
