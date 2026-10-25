<script setup lang="ts">
import { computed, ref, shallowRef } from 'vue'
import { useAtelier } from '../app/store'
import { service } from '../services/mock'
import type { Preview } from '../types/atelier'
import Panel from '../shared/components/Panel.vue'
import Modal from '../shared/components/Modal.vue'
import Icon from '../shared/components/Icon.vue'
const opener = shallowRef<HTMLElement | null>(null)
const store = useAtelier(),
  filter = ref('all'),
  selected = ref('provider'),
  preview = ref<Preview | null>(null),
  busy = ref(false),
  confirmed = ref(false),
  failure = ref('')
const checks = computed(() =>
    store.checks.filter((c) => filter.value === 'all' || c.status === filter.value),
  ),
  detail = computed(() => checks.value.find((c) => c.id === selected.value) ?? checks.value[0])

async function open(id: string) {
  if (busy.value) return
  opener.value = document.activeElement as HTMLElement
  busy.value = true
  failure.value = ''
  try {
    preview.value = await service.preview(id)
    confirmed.value = false
  } catch {
    failure.value = '预览加载失败，请重试。'
  } finally {
    busy.value = false
  }
}
async function confirm() {
  if (!preview.value || !confirmed.value) return
  busy.value = true
  try {
    await service.confirm(preview.value)
    preview.value = null
    store.notify('模拟完成 · 未写入配置，也未创建真实备份')
  } catch {
    failure.value = '模拟失败，请重试。'
  } finally {
    busy.value = false
  }
}
</script>
<template>
  <div class="page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">WORKSPACE / DIAGNOSTICS</p>
        <h1>每一次调整，都有退路<span class="heading-dot">.</span></h1>
        <p class="subtitle">检查、预览、确认。让环境变化始终清晰可控。</p>
      </div>
      <button
        class="button"
        :disabled="store.loading"
        @click="store.load().then(() => store.notify('演示检查完成，未扫描本机'))"
      >
        <Icon name="refresh" :size="16" />{{ store.loading ? '模拟检查中…' : '运行演示检查' }}
      </button>
    </div>
    <div class="diagnostic-banner">
      <Icon name="shield" :size="21" /><span
        ><b>演示检查报告</b>以下结果来自固定示例，未扫描电脑。修复和恢复只模拟流程。</span
      ><span class="mono">READ ONLY / MOCK</span>
    </div>
    <div class="diagnostics-layout">
      <Panel title="分类检查" code="HEALTH CHECK"
        ><div class="tabs compact">
          <button
            v-for="f in [
              { id: 'all', label: '全部检查' },
              { id: 'warning', label: '待处理' },
              { id: 'pass', label: '已通过' },
            ]"
            :key="f.id"
            :class="{ selected: filter === f.id }"
            :aria-pressed="filter === f.id"
            @click="filter = f.id"
          >
            {{ f.label }}
          </button>
        </div>
        <button
          v-for="c in checks"
          :key="c.id"
          class="check-row"
          :class="{ selected: detail?.id === c.id }"
          :aria-pressed="detail?.id === c.id"
          @click="selected = c.id"
        >
          <span class="check-status" :class="c.status"
            ><Icon :name="c.status === 'pass' ? 'check' : 'alert'" :size="19" /></span
          ><span
            ><small>{{ c.group }}</small
            ><b>{{ c.title }}</b></span
          ><span class="badge" :class="{ orange: c.status === 'warning' }">{{
            c.status === 'pass' ? '通过' : '建议处理'
          }}</span
          ><Icon name="chevron" :size="14" /></button></Panel
      ><Panel title="问题说明" code="DETAIL"
        ><template v-if="detail"
          ><span class="eyebrow">{{ detail.group }} / DEMO</span>
          <h2>{{ detail.title }}</h2>
          <p class="muted">{{ detail.detail }}</p>
          <div class="inset risk-box">
            <b><Icon name="alert" :size="15" />风险与边界</b>
            <p>{{ detail.risk }}</p>
          </div>
          <button
            class="button primary"
            :disabled="busy || !store.demo || detail.status === 'pass'"
            @click="open(detail.id)"
          >
            <Icon name="terminal" :size="16" />{{ busy ? '正在准备…' : '预览演示修复'
            }}<Icon name="arrow" :size="16" />
          </button>
          <p class="tiny muted">
            {{
              detail.status === 'pass' ? '此项无需修复。' : '确认之前先查看差异，当前不会写入系统。'
            }}
          </p></template
        ></Panel
      >
    </div>
    <Panel title="备份档案" code="BACKUP ARCHIVE / 示例"
      ><div class="backup-intro">
        <span>每个关键变更之前，留下一份可恢复的快照。</span
        ><span class="badge neutral">3 份演示备份</span>
      </div>
      <div class="backup-row" v-for="b in store.backups" :key="b.id">
        <span class="square-icon"><Icon name="history" /></span>
        <div>
          <b>{{ b.name }}</b
          ><small class="mono">{{ b.date }} · {{ b.size }}</small>
        </div>
        <span class="badge">示例校验通过</span
        ><button class="button" :disabled="busy || !store.demo" @click="open(b.id)">
          预览恢复 <Icon name="arrow" :size="14" />
        </button></div
    ></Panel>
    <p v-if="failure" role="alert" class="error-state">{{ failure }}</p>
    <Modal
      v-if="preview"
      :return-focus="opener"
      title="变更预览 / DEMO TRANSACTION"
      @close="!busy && (preview = null)"
      ><span class="badge orange">仅模拟 · 不写入本机</span>
      <h2>{{ preview.id.startsWith('b') ? '恢复演示备份' : '调整演示配置' }}</h2>
      <p class="muted">以下是虚构配置差异。确认不会触及真实配置、代理或凭据。</p>
      <div class="diff">
        <div>
          <span>BEFORE / 修改前</span>
          <pre>{{ preview.before }}</pre>
        </div>
        <div>
          <span>AFTER / 修改后</span>
          <pre>{{ preview.after }}</pre>
        </div>
      </div>
      <ol class="transaction-steps">
        <li
          v-for="step in [
            '读取示例快照',
            '检测版本变化（模拟）',
            '创建演示备份',
            '确认后模拟写入',
            '模拟校验与回滚入口',
          ]"
          :key="step"
        >
          {{ step }}
        </li>
      </ol>
      <label class="confirm-checkbox"
        ><input type="checkbox" v-model="confirmed" />我理解这只运行演示，不会修复真实环境。</label
      >
      <p v-if="failure" role="alert">{{ failure }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="busy" @click="preview = null">取消</button
        ><button
          class="button primary"
          :disabled="!confirmed || busy || !store.demo"
          @click="confirm"
        >
          {{ busy ? '正在模拟…' : '确认模拟' }}
        </button>
      </div></Modal
    >
  </div>
</template>
