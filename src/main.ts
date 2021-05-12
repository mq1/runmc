import { ViteSSG } from 'vite-ssg'
import { setupLayouts } from 'virtual:generated-layouts'
import generatedRoutes from 'virtual:generated-pages'
import App from './App.vue'
import 'virtual:windi.css'
import './main.css'

const routes = setupLayouts(generatedRoutes)

export const createApp = ViteSSG(App, { routes })
