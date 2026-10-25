import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { router } from './app/router'
import './styles/tokens/theme.css'
import './styles/main.css'
import App from './App.vue'
createApp(App).use(createPinia()).use(router).mount('#app')
