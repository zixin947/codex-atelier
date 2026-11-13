import { createRouter, createWebHashHistory } from 'vue-router'
import Maintenance from '../features/maintenance/Maintenance.vue'
import Threads from '../features/threads/Threads.vue'
import Workbench from '../features/workbench/Workbench.vue'
import DesktopPets from '../features/desktop-pets/DesktopPets.vue'
import BehaviorPlugins from '../features/plugins/BehaviorPlugins.vue'
import Accounts from '../features/accounts/Accounts.vue'
import OfficialAccounts from '../features/accounts/OfficialAccounts.vue'
import NativeModels from '../features/connections/NativeModels.vue'

/* 正式页面全部走真实管理能力。演示扩展、演示诊断与规划中人设已从导航移除，
   旧链接统一落到各自最接近的真实页面，未知路径回首页。 */
export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: Workbench, props: { tab: 'usage' } },
    { path: '/workbench/requests', component: Workbench, props: { tab: 'requests' } },
    { path: '/workbench/sessions', component: Workbench, props: { tab: 'sessions' } },
    { path: '/workbench/context', component: Workbench, props: { tab: 'context' } },
    { path: '/threads', component: Threads },
    { path: '/desktop-pets', component: DesktopPets },
    { path: '/plugins', component: BehaviorPlugins },
    { path: '/accounts', component: Accounts },
    { path: '/accounts/official', component: OfficialAccounts },
    {
      path: '/connections',
      component: NativeModels,
      props: { view: 'agents', title: '模型与连接' },
    },
    { path: '/providers', component: NativeModels, props: { view: 'providers', title: '供应商' } },
    { path: '/routing', component: NativeModels, props: { view: 'routing', title: '路由' } },
    {
      path: '/routing-groups',
      component: NativeModels,
      props: { view: 'groups', title: '路由组' },
    },
    { path: '/gateway', component: NativeModels, props: { view: 'gateway', title: '网关' } },
    { path: '/maintenance', component: Maintenance },
    { path: '/planned/history', redirect: '/threads' },
    { path: '/planned/accounts', redirect: '/accounts' },
    { path: '/planned/models', redirect: '/connections' },
    { path: '/:pathMatch(.*)*', redirect: '/' },
  ],
})
