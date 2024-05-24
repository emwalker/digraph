export const ROOT_USER_ID = '2db58326-ddfa-4561-9ae2-232aa5c32277'

export type ErrorMap = { [key:string]: string[] }

export type User = {
  id: string,
  username: string,
  name: string | null,
  is_admin: boolean,
}

export type FetchUsersResponse = {
  items: User[],
  page: number,
  per_page: number,
  total: number,
}

export async function fetchUsers(page: number, perPage: number): Promise<FetchUsersResponse> {
  const url = new URL('http://localhost:3002/api/users')
  url.searchParams.set('page', page.toString())
  url.searchParams.set('per_page', perPage.toString())
  const res = await fetch(url, { cache: 'no-cache' })

  if (!res.ok) {
    throw new Error('Failed to fetch users')
  }

  return res.json()
}

export type FetchUserResponse = {
  user: User | null,
}

export async function fetchUser(userId: string): Promise<FetchUserResponse> {
  const res = await fetch(`http://localhost:3002/api/users/${userId}`, { cache: 'no-cache' })

  if (!res.ok) {
    console.log('failed to fetch user: ', res)
  }

  return res.json()
}

type CreateUserPayload = {
  username: string,
}

type CreateUserResponse = {
  user_id: string | null,
  errors: ErrorMap,
  created: boolean,
}

export async function createUser(payload: CreateUserPayload): Promise<CreateUserResponse> {
  const res = await fetch('/api/users', {
    headers: { 'Content-Type': 'application/json' },
    method: 'POST',
    body: JSON.stringify(payload),
  })

  if (!res.ok) {
    console.log('failed to create user: ', res)
  }

  return res.json()
}

export type Topic = {
  id: string,
  name: string,
}

export type FetchTopicsResponse = {
  items: Topic[],
  page: number,
  per_page: number,
  total: number,
}

export async function fetchTopics(page: number, perPage: number): Promise<FetchTopicsResponse> {
  const url = new URL('http://localhost:3002/api/topics')
  url.searchParams.set('page', page.toString())
  url.searchParams.set('per_page', perPage.toString())
  const res = await fetch(url, { cache: 'no-cache' })

  if (!res.ok) {
    throw new Error('Failed to fetch topics')
  }

  return res.json()
}

export type FetchTopicResponse = {
  topic: Topic | null,
}

export async function fetchTopic(topicId: string): Promise<FetchTopicResponse> {
  const res = await fetch(`http://localhost:3002/api/topics/${topicId}`, { cache: 'no-cache' })

  if (!res.ok) {
    console.log('failed to fetch topic: ', res)
  }

  return res.json()
}

type CreateTopicPayload = {
  owner_id: string,
  name: string,
}

type CreateTopicResponse = {
  topic_id: string | null,
  errors: ErrorMap,
  created: boolean,
}

export async function createTopic(payload: CreateTopicPayload): Promise<CreateTopicResponse> {
  const res = await fetch('/api/topics', {
    headers: { 'Content-Type': 'application/json' },
    method: 'POST',
    body: JSON.stringify(payload),
  })

  if (!res.ok) {
    console.log('failed to create topic: ', res)
  }

  return res.json()
}
