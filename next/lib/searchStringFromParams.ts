import { ParsedUrlQuery } from 'querystring'
import { ROOT_TOPIC_ID } from './constants'

export const searchStringFromParams = (
  params: ParsedUrlQuery | undefined
): string => {
  if (!params) return `in:${ROOT_TOPIC_ID}`

  const { q, id } = params
  const parentTopicId = id || ROOT_TOPIC_ID
  if (!q) return `in:${parentTopicId}`
  return `in:${parentTopicId} ${decodeURIComponent(q as string)}`
}
