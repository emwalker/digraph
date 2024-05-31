import { ParsedUrlQuery } from 'querystring'
import { ROOT_TOPIC_ID } from './constants'

export const searchStringFromParams = (
  params: ParsedUrlQuery | undefined
): string => {
  if (!params) return `in:${ROOT_TOPIC_ID}`

  const { q, id } = params
  const parentTopicId = id == null ? ROOT_TOPIC_ID : params.id
  return q == null ? `in:${parentTopicId}` : `in:${parentTopicId} ${q}`
}
