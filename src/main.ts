import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { router } from './app/router'
import './styles/tokens/theme.css'
import './styles/tokens/semantic.css'
import './styles/main.css'
import App from './App.vue'
import { initializeTheme } from './app/theme'
import { startStartupAnimation } from './app/startup'
import './styles/themes/endfield.css'
import './styles/themes/endfield-startup.css'
import './styles/themes/endfield-polish.css'
import './styles/themes/hud.css'
initializeTheme()
startStartupAnimation()
createApp(App).use(createPinia()).use(router).mount('#app')
