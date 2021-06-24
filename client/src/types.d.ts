/* eslint-disable no-unused-vars */

export type State = {
  authenticated?: boolean,
  authenticate: (update: boolean, data: Api.AuthData | undefined) => Promise<boolean>
  logout: () => void,
  fetch: () => Promise<boolean>
  loading: boolean,
  userData: Api.User | undefined
}

declare namespace Api {
  export type User = {
    username: string,
    email: string,
    id: string
  }

  export type AuthData = {
    id: string,
    password: string,
    remember: boolean
  }
}
