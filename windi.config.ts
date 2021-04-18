import { defineConfig } from 'vite-plugin-windicss'

export default defineConfig({
  darkMode: 'media',
  plugins: [
    require('windicss/plugin/forms'),
  ],
})
