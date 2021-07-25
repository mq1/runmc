import { ViteSSGContext } from 'vite-ssg'

export type UserModule = (ctx: ViteSSGContext) => void

export type Account = {
  name: string
  id: string
  accessToken: string
}

export type InstanceInfo = {
  gameVersion: string
  mainClass: string
  fabric: boolean
}

export type Config = {
  java: {
    path: string
    memory: string
  }
  locale: string
}
