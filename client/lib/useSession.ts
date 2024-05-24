// https://github.com/vvo/iron-session/blob/77dfa0af8c6b91c52622f9efd4443f4d292a5e05/examples/next/src/app/app-router-client-component-route-handler-swr/use-session.ts
import useSWR from 'swr'
import useSWRMutation from 'swr/mutation'
import { SessionData, defaultSession } from './index'

const sessionApiRoute = '/session'

async function fetchJson<JSON = unknown>(
  input: RequestInfo,
  init?: RequestInit,
): Promise<JSON> {
  return fetch(input, {
    headers: {
      accept: 'application/json',
      'content-type': 'application/json',
    },
    ...init,
  }).then((res) => res.json())
}

function doLogin(url: string, { arg }: { arg: string }) {
  return fetchJson<SessionData>(url, {
    method: 'POST',
    body: JSON.stringify({ username: arg }),
  })
}

function doLogout(url: string) {
  return fetchJson<SessionData>(url, {
    method: 'DELETE',
  })
}

export default function useSession() {
  const { data: session, isLoading } = useSWR(
    sessionApiRoute,
    fetchJson<SessionData>,
    {
      fallbackData: defaultSession,
    },
  )

  const { trigger: login } = useSWRMutation(sessionApiRoute, doLogin, {
    // the login route already provides the updated information, no need to revalidate
    revalidate: false,
  })
  const { trigger: logout } = useSWRMutation(sessionApiRoute, doLogout)

  return { session, logout, login, isLoading }
}
