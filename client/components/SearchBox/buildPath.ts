import { SearchBoxQuery } from '@/lib/__generated__/graphql'
import { ROOT_TOPIC_ID } from '@/lib/constants'

type QueryInfo = SearchBoxQuery['view']['queryInfo']

export function buildPath(
  searchTerms: string[],
  queryInfo: QueryInfo,
  newQueryInfo: Map<string, string>
): string {
  const displayNamesToIds: Map<string, string> = new Map(newQueryInfo)

  queryInfo.topics.forEach((topic) => {
    displayNamesToIds.set(topic.displayName, topic.id)
  })

  const topicIds = searchTerms.map((name) => displayNamesToIds.get(name)).filter(Boolean)
  const parentTopicId = topicIds.length < 1 ? ROOT_TOPIC_ID : topicIds[0]
  const phrases = searchTerms.filter((name) => !displayNamesToIds.has(name))
  const q = topicIds.slice(1).map((id) => `in:${id}`).concat(phrases).join(' ')

  if (q.length > 0) {
    return `/topics/${parentTopicId}?q=${q}`
  }

  return `/topics/${parentTopicId}`
}
