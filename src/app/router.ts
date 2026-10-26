import { createRouter, createWebHashHistory } from 'vue-router'
import Dashboard from '../features/Dashboard.vue'
import Extensions from '../features/Extensions.vue'
import Diagnostics from '../features/Diagnostics.vue'
import Planned from '../features/Planned.vue'
import BehaviorPlugins from '../features/plugins/BehaviorPlugins.vue'
export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: Dashboard },
    { path: '/extensions', component: Extensions },
    { path: '/plugins', component: BehaviorPlugins },
    { path: '/diagnostics', component: Diagnostics },
    { path: '/planned/:module', component: Planned },
  ],
})
