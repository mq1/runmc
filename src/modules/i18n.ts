import { createI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/tauri'
import { Config, UserModule } from '~/types'

// import i18n resources
// https://vitejs.dev/guide/features.html#glob-import
const messages = Object.fromEntries(
  Object.entries(
    import.meta.globEager('../../locales/*.y(a)?ml'))
    .map(([key, value]) => {
      const yaml = key.endsWith('.yaml')
      return [key.slice(14, yaml ? -5 : -4), value.default]
    }),
)

const config: Config = await invoke('get_config')

export const install: UserModule = ({ app }) => {
  const i18n = createI18n({
    legacy: false,
    locale: config.locale,
    messages,
  })

  app.use(i18n)
}
