export type Account = {
  name: string
  id: string
  accessToken: string
}

export type Version = {
  id: string
  type: string
  url: string
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
}
