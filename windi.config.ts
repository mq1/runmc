import { defineConfig } from 'vite-plugin-windicss'

export default defineConfig({
  darkMode: 'media',
  theme: {
    fontFamily: {
      sans: ['Inter var', 'sans-serif'],
    },
  },
  plugins: [
    require('windicss/plugin/typography'),
    require('windicss/plugin/forms'),
  ],
})
