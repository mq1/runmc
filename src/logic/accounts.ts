import { readTextFile, writeFile, BaseDirectory } from '@tauri-apps/api/fs'
import { fetch, Body, ResponseType } from '@tauri-apps/api/http'
import * as yaml from 'js-yaml'

const path = 'accounts.yml'

export type Account = {
  name: string
  id: string
  accessToken: string
}

type Config = {
  clientToken: string | null
  list: Account[]
}

const newConfig = async() => {
  const accounts: Config = { clientToken: null, list: [] }
  const contents = yaml.dump(accounts)
  await writeFile({ contents, path }, { dir: BaseDirectory.App })

  return accounts
}

const read = () =>
  readTextFile(path, { dir: BaseDirectory.App })
    .then(contents => yaml.load(contents) as Config)
    .catch(() => newConfig())

const write = async(accounts: Config) => {
  const contents = yaml.dump(accounts)
  await writeFile({ contents, path }, { dir: BaseDirectory.App })
}

export const list = () =>
  read()
    .then(accounts => accounts.list)
    .catch(() => [])

const add = async(account: Account, clientToken: string) => {
  const accounts = await read()
  accounts.list.push(account)
  accounts.clientToken = clientToken

  await write(accounts)
}

const remove = async({ id }: Account) => {
  const accounts = await read()
  accounts.list = accounts.list.filter(a => a.id !== id)

  await write(accounts)
}

const postJSON = (url: string, data: any) =>
  fetch<any>(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: Body.json(data),
    responseType: ResponseType.JSON,
  })

export const authenticate = async(email: string, password: string) => {
  const accounts = await read()

  console.log(`Trying to add account ${email}`)

  const payload = {
    username: email,
    password,
    agent: {
      name: 'Minecraft',
      version: 1,
    },
    clientToken: accounts.clientToken,
  }

  const response = await postJSON('https://authserver.mojang.com/authenticate', payload)
  const { selectedProfile, accessToken, clientToken } = await response.data

  const account: Account = {
    name: selectedProfile.name,
    id: selectedProfile.id,
    accessToken,
  }

  await add(account, clientToken)
}

export const refresh = async(account: Account) => {
  const { clientToken } = await read()

  const payload = {
    accessToken: account.accessToken,
    clientToken,
  }

  const response = await postJSON('https://authserver.mojang.com/refresh', payload)
  const { accessToken } = await response.data

  const newAccount = account
  newAccount.accessToken = accessToken

  // update accounts
  remove(account)
  add(newAccount, clientToken!)

  return newAccount
}

export const validate = async({ accessToken }: Account) => {
  const { clientToken } = await read()

  const payload = {
    accessToken,
    clientToken,
  }

  const response = await postJSON('https://authserver.mojang.com/validate', payload)

  return response.status === 204
}

export const invalidate = async(account: Account) => {
  const { clientToken } = await read()

  const payload = {
    accessToken: account.accessToken,
    clientToken,
  }

  const response = await postJSON('https://authserver.mojang.com/invalidate', payload)

  await remove(account)

  return response.status === 204
}
