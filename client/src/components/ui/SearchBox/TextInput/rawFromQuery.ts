import { RawDraftContentState } from 'draft-js'

import { SearchBox_view$data as ViewType } from '__generated__/SearchBox_view.graphql'

type QueryInfo = ViewType['queryInfo']

type Range = {
  key: number,
  length: number,
  offset: number,
}

const rawFromQuery = (queryInfo: QueryInfo, genKey: Function): RawDraftContentState => {
  const { topics: queryTopics, phrases } = queryInfo
  const entityRanges: Range[] = []
  const entityMap: { [key: number]: any } = {}
  const tokens: string[] = []
  let entityIndex = 0
  let startLast = 0

  queryTopics.forEach((node) => {
    if (node != null) {
      const { displayName, id } = node
      entityRanges.push({
        key: entityIndex,
        length: displayName.length,
        offset: startLast,
      })

      entityMap[entityIndex] = {
        type: 'in:mention',
        mutability: 'SEGMENTED',
        data: {
          mention: {
            displayName,
            link: id,
          },
        },
      }

      tokens.push(displayName)
      startLast += displayName.length + 1
      entityIndex += 1
    }
  })

  const text = [...tokens, ...phrases].join(' ')

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
