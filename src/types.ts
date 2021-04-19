export type Account = {
  name: string
  id: string
  access_token: string
}

export type Version = {
  id: string
  type: string
  url: string
}

export type Config = {
  java_path: string
  java_memory_mb: number
}
