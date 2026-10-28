import { createRouter, createWebHashHistory } from 'vue-router'
import Dashboard from '../features/Dashboard.vue'
import Extensions from '../features/Extensions.vue'
import Diagnostics from '../features/Diagnostics.vue'
import Planned from '../features/Planned.vue'
import BehaviorPlugins from '../features/plugins/BehaviorPlugins.vue'
import Accounts from '../features/accounts/Accounts.vue'
import OfficialAccounts from '../features/accounts/OfficialAccounts.vue'
import Connections from '../features/connections/Connections.vue'
export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: Dashboard },
    { path: '/extensions', component: Extensions },
    { path: '/plugins', component: BehaviorPlugins },
    { path: '/accounts', component: Accounts },
    { path: '/accounts/official', component: OfficialAccounts },
    { path: '/connections', component: Connections },
    { path: '/planned/accounts', redirect: '/accounts' },
    { path: '/planned/models', redirect: '/connections' },
    { path: '/diagnostics', component: Diagnostics },
    { path: '/planned/:module', component: Planned },
  ],
})
