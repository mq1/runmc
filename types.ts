export type MinecraftVersion = {
  id: string
  type: string
  url: string
}

export type InstanceConfig = {
  minecraftVersion: string
  type: string
}

export type Config = {
  java: {
    path: string
    memory: string
  }
  locale: string
}
