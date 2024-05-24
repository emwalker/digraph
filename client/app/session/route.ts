// https://github.com/vvo/iron-session/blob/77dfa0af8c6b91c52622f9efd4443f4d292a5e05/examples/next/src/app/app-router-client-component-route-handler-swr/session/route.ts
import { NextRequest } from 'next/server'
import { cookies } from 'next/headers'
import { getIronSession } from 'iron-session'
import { defaultSession, sessionOptions, sleep, SessionData } from '@/lib'

// login
export async function POST(request: NextRequest) {
  const session = await getIronSession<SessionData>(cookies(), sessionOptions)

  const { username = 'No username' } = (await request.json()) as {
    username: string;
  }

  session.isLoggedIn = true
  session.username = username
  await session.save()

  // simulate looking up the user in db
  await sleep(250)

  return Response.json(session)
}

// read session
export async function GET() {
  const session = await getIronSession<SessionData>(cookies(), sessionOptions)

  // simulate looking up the user in db
  await sleep(250)

  if (session.isLoggedIn !== true) {
    return Response.json(defaultSession)
  }

  return Response.json(session)
}

// logout
export async function DELETE() {
  const session = await getIronSession<SessionData>(cookies(), sessionOptions)

  session.destroy()

  return Response.json(defaultSession)
}
