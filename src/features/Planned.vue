<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import Icon from '../shared/components/Icon.vue'
const route = useRoute()
const modules: Record<
  string,
  { title: string; en: string; icon: string; description: string; items: string[] }
> = {
  accounts: {
    title: '账号与额度',
    en: 'ACCOUNTS & USAGE',
    icon: 'user',
    description: '让每个账号的使用情况，清楚地待在同一个地方。',
    items: ['账号档案与切换', '额度与重置时间', '凭据安全边界'],
  },
  models: {
    title: '模型与连接',
    en: 'MODELS & CONNECTIONS',
    icon: 'chip',
    description: '为不同的工作，准备恰到好处的模型与连接。',
    items: ['模型与 Provider 档案', '代理路由可视化', '连接检查与变更预览'],
  },
  history: {
    title: '会话与历史',
    en: 'SESSIONS & HISTORY',
    icon: 'history',
    description: '让值得保留的思路，有迹可循。',
    items: ['会话检索与归档', '工作区关联', '只读导入预览'],
  },
  companions: {
    title: '人设与陪伴',
    en: 'PERSONA & COMPANIONS',
    icon: 'heart',
    description: '一个有温度的工作间，也要有清晰的边界。',
    items: ['人设配置与版本', '长期记忆管理', '桌宠与陪伴空间'],
  },
}
const module = computed(() => modules[String(route.params.module)] ?? modules.accounts!)
</script>
<template>
  <div class="page">
    <p class="eyebrow">WORKSPACE / {{ module.en }}</p>
    <div class="planned">
      <div class="planned-mark"><Icon :name="module.icon" :size="48" /></div>
      <span class="badge orange">ON THE DRAWING BOARD</span>
      <h1>{{ module.title }}</h1>
      <p>{{ module.description }}</p>
      <div class="planned-roadmap">
        <div v-for="(item, i) in module.items" :key="item">
          <span class="mono">0{{ i + 1 }}</span
          >{{ item }}<span class="tiny">规划中</span>
        </div>
      </div>
      <div class="inset safety-note">
        此模块尚未实现，未接入真实数据或配置。<br />当前可体验主工作台、扩展管理与诊断演示。
      </div>
      <RouterLink class="button primary" to="/"
        >返回工作台 <Icon name="arrow" :size="16"
      /></RouterLink>
    </div>
  </div>
</template>
