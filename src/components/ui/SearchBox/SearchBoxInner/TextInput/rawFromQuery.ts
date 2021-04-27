import { RawDraftContentState } from 'draft-js'

import { SearchBox_view as ViewType } from '__generated__/SearchBox_view.graphql'

type QueryInfo = ViewType['queryInfo']

type Range = {
  key: number,
  length: number,
  offset: number,
}

const rawFromQuery = (queryInfo: QueryInfo, genKey: Function): RawDraftContentState => {
  const { topics: queryTopics, stringTokens } = queryInfo
  const entityRanges: Range[] = []
  const entityMap: { [key: number]: any } = {}
  const tokens: string[] = []
  const topics = queryTopics?.edges || []
  let entityIndex = 0
  let startLast = 0

  topics.forEach((edge) => {
    const node = edge?.node
    if (node != null) {
      const { name, resourcePath } = node
      entityRanges.push({
        key: entityIndex,
        length: name.length,
        offset: startLast,
      })

      entityMap[entityIndex] = {
        type: 'in:mention',
        mutability: 'SEGMENTED',
        data: {
          mention: {
            name,
            link: resourcePath,
          },
        },
      }

      tokens.push(name)
      startLast += name.length + 1
      entityIndex += 1
    }
  })

  const text = [...tokens, ...stringTokens].join(' ')

  return {
    blocks: [
      {
        data: {},
        depth: 0,
        inlineStyleRanges: [],
        entityRanges,
        key: genKey(),
        text,
        type: 'unstyled',
      },
    ],
    entityMap,
  }
}

export default rawFromQuery
