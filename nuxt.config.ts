import { defineNuxtConfig } from 'nuxt3'

// https://v3.nuxtjs.org/docs/directory-structure/nuxt.config
export default defineNuxtConfig({
  buildModules: [
    '@unocss/nuxt',
    '@intlify/nuxt3',
    '@vueuse/nuxt',
  ],
  unocss: {
    uno: true,
    attributify: true,
    preflight: true,
    icons: true,
  },
})
