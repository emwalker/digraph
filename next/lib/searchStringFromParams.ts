import { ReadonlyURLSearchParams } from 'next/navigation'
import { Params } from 'next/dist/shared/lib/router/utils/route-matcher'
import { ROOT_TOPIC_ID } from './constants'

export const searchStringFromParams = (
  params: Params, searchParams: ReadonlyURLSearchParams
): string => {
  const parentTopicId = params.id == null ? ROOT_TOPIC_ID : params.id
  const q = searchParams.get('q')
  return q == null ? `in:${parentTopicId}` : `in:${parentTopicId} ${q}`
}
