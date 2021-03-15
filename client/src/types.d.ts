/* eslint-disable no-unused-vars */

export type State = {
  authenticated?: boolean,
  authenticate: (id: string, password: string, remember: boolean) => Promise<boolean>
  logout: () => void
  loading: boolean,
}

declare namespace Api {
  export type User = {
    username: string,
    email: string,
    id: string
  }
}
