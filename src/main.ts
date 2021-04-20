import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import { setupLayouts } from 'layouts-generated'
import generatedRoutes from 'virtual:generated-pages'
import App from './App.vue'
import 'virtual:windi.css'

const routes = setupLayouts(generatedRoutes)

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

const app = createApp(App)

app.use(router)

app.mount('#app')
