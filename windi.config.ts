import { defineConfig } from 'vite-plugin-windicss'

export default defineConfig({
  darkMode: 'media',
  theme: {
    extend: {
      typography: {
        light: {
          css: [
            {
              'color': '#9ca3af', // gray-400
              '[class~="lead"]': {
                color: '#d1d5db', // gray-300
              },
              'a': {
                color: '#fff', // white
              },
              'strong': {
                color: '#fff', // white
              },
              'ol > li::before': {
                color: '#9ca3af', // gray-400
              },
              'ul > li::before': {
                backgroundColor: '#4b5563', // gray-600
              },
              'hr': {
                borderColor: '#e5e7eb', // gray-200
              },
              'blockquote': {
                color: '#e5e7eb', // gray-200
                borderLeftColor: '#4b5563', // gray-600
              },
              'h1': {
                color: '#fff', // white
              },
              'h2': {
                color: '#fff', // white
              },
              'h3': {
                color: '#fff', // white
              },
              'h4': {
                color: '#fff', // white
              },
              'figure figcaption': {
                color: '#9ca3af', // gray-400
              },
              'code': {
                color: '#fff', // white
              },
              'a code': {
                color: '#fff', // white
              },
              'pre': {
                color: '#e5e7eb', // gray-200
                backgroundColor: '#1f2937', // gray-800
              },
              'thead': {
                color: '#fff', // white
                borderBottomColor: '#9ca3af', // gray-400
              },
              'tbody tr': {
                borderBottomColor: '#4b5563', // gray-600
              },
            },
          ],
        },
      },
    },
  },
  plugins: [
    require('windicss/plugin/typography'),
    require('windicss/plugin/forms'),
  ],
})
