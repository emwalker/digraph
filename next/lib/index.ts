// https://github.com/vvo/iron-session/blob/77dfa0af8c6b91c52622f9efd4443f4d292a5e05/examples/next/src/app/app-router-client-component-route-handler-swr/lib.ts
import { SessionOptions } from 'iron-session'

export interface SessionData {
  username: string;
  isLoggedIn: boolean;
}

export const defaultSession: SessionData = {
  username: '',
  isLoggedIn: false,
}

export const sessionOptions: SessionOptions = {
  password: 'c64cd57aa009e702fea94b8f175a72734c4144a3',
  cookieName: 'links',
  cookieOptions: {
    secure: process.env.NODE_ENV === 'production',
  },
}

export function sleep(ms: number) {
  return new Promise((resolve) => { setTimeout(resolve, ms) })
}
