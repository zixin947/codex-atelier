<script setup lang="ts">
import { computed, ref } from 'vue'
import { useAtelier } from '../app/store'
import Panel from '../shared/components/Panel.vue'
import Icon from '../shared/components/Icon.vue'
import type { Kind } from '../types/atelier'
const store = useAtelier(),
  kind = ref<Kind>('MCP'),
  query = ref(''),
  status = ref('all'),
  selected = ref('filesystem'),
  busy = ref(false),
  error = ref('')
const filtered = computed(() =>
  store.extensions.filter(
    (e) =>
      e.kind === kind.value &&
      `${e.name} ${e.subtitle}`.toLowerCase().includes(query.value.toLowerCase()) &&
      (status.value === 'all' || e.enabled === (status.value === 'enabled')),
  ),
)
const detail = computed(
  () => filtered.value.find((e) => e.id === selected.value) ?? filtered.value[0],
)
function resetFilters() {
  query.value = ''
  status.value = 'all'
}
async function toggle() {
  if (!detail.value) return
  busy.value = true
  error.value = ''
  try {
    await store.toggle(detail.value.id)
  } catch {
    error.value = '示例更新失败，请重试。'
  } finally {
    busy.value = false
  }
}
</script>
<template>
  <div class="page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">WORKSPACE / EXTENSIONS</p>
        <h1>为工作流，扩展可能<span class="heading-dot">.</span></h1>
        <p class="subtitle">所有开关仅修改演示状态，不影响本机 Codex。</p>
      </div>
      <span class="badge orange">MOCK REGISTRY</span>
    </div>
    <div class="toolbar">
      <div class="tabs" aria-label="扩展分类">
        <button
          v-for="k in ['MCP', 'Skills'] as const"
          :key="k"
          :class="{ selected: kind === k }"
          :aria-pressed="kind === k"
          @click="kind = k"
        >
          {{ k === 'MCP' ? 'MCP 服务器' : 'Skills 技能'
          }}<span>{{ store.extensions.filter((e) => e.kind === k).length }}</span>
        </button>
      </div>
      <label class="search"
        ><Icon name="search" :size="17" /><input
          v-model="query"
          aria-label="搜索扩展"
          placeholder="搜索名称或用途…" /></label
      ><select v-model="status" aria-label="状态筛选">
        <option value="all">全部状态</option>
        <option value="enabled">已启用</option>
        <option value="disabled">已停用</option>
      </select>
    </div>
    <div class="extensions-layout">
      <section class="extension-list" aria-label="扩展列表">
        <div v-if="store.loading" class="empty">正在加载示例扩展…</div>
        <button
          v-for="e in filtered"
          :key="e.id"
          class="extension-card"
          :class="{ selected: detail?.id === e.id }"
          :aria-pressed="detail?.id === e.id"
          @click="selected = e.id"
        >
          <div class="extension-card-top">
            <span class="square-icon"
              ><Icon :name="e.kind === 'MCP' ? 'box' : 'book'" :size="24" /></span
            ><span class="badge" :class="{ neutral: !e.enabled }"
              ><i class="dot" />{{ e.enabled ? '已启用' : '已停用' }}</span
            >
          </div>
          <h3>{{ e.name }}</h3>
          <p>{{ e.subtitle }}</p>
          <div class="extension-card-footer">
            <span class="mono">v{{ e.version }} · 示例</span><Icon name="arrow" :size="16" />
          </div>
        </button>
        <div v-if="!store.loading && !filtered.length" class="empty">
          <Icon name="search" :size="30" />
          <h3>没有找到匹配扩展</h3>
          <p>试试其他关键词，或重置筛选。</p>
          <button class="button" @click="resetFilters">重置筛选</button>
        </div>
      </section>
      <Panel title="扩展详情" code="INSPECTOR"
        ><template v-if="detail"
          ><div class="detail-heading">
            <span class="square-icon"
              ><Icon :name="detail.kind === 'MCP' ? 'box' : 'book'" :size="27" /></span
            ><span class="badge neutral">{{ detail.kind }}</span>
          </div>
          <h2>{{ detail.name }}</h2>
          <p class="muted">{{ detail.description }}</p>
          <dl class="detail-meta">
            <dt>来源 · 示例</dt>
            <dd>Atelier demo registry</dd>
            <dt>版本</dt>
            <dd class="mono">{{ detail.version }}</dd>
            <dt>运行位置</dt>
            <dd>演示沙盒</dd>
          </dl>
          <div class="setting-row">
            <span><b>启用此扩展</b><small>仅作用于本次演示</small></span
            ><button
              class="switch"
              :class="{ on: detail.enabled }"
              role="switch"
              :aria-checked="detail.enabled"
              :aria-label="'启用 ' + detail.name"
              :disabled="busy || !store.demo"
              @click="toggle"
            >
              <span />
            </button>
          </div>
          <p v-if="error" role="alert" class="error-state">{{ error }}</p>
          <p class="eyebrow">权限声明 / 示例</p>
          <div class="permission">
            <Icon name="shield" :size="14" />{{
              detail.kind === 'MCP' ? '限定工作区访问（示例）' : '无额外权限（示例）'
            }}
          </div>
          <p class="eyebrow">{{ detail.kind === 'MCP' ? 'TOOLS' : 'WORKFLOW' }}</p>
          <div class="tool-tags">
            <code v-for="t in detail.tools" :key="t">{{ t }}</code>
          </div>
          <div class="inset safety-note">
            此处不安装、不执行扩展，也不读取真实 MCP 或 Skills 配置。
          </div></template
        >
        <div v-else class="empty">选择一个扩展以查看详情</div></Panel
      >
    </div>
  </div>
</template>
