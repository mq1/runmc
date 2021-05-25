import { defineConfig } from 'vite-plugin-windicss'
import colors from 'windicss/colors'

export default defineConfig({
  darkMode: 'media',
  theme: {
    extend: {
      colors: {
        primary: colors.purple,
      },
    },
  },
  plugins: [
    require('windicss/plugin/forms'),
  ],
})
