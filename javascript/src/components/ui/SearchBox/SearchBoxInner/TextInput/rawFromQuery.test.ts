import rawFromQuery from './rawFromQuery'

describe('rawFromQuery', () => {
  it('handles a simple case', () => {
    const genKey = () => 'key'

    const queryInfo = {
      topics: {
        edges: [
          {
            node: {
              name: 'BP',
              path: '/wiki/topics/321ccdae-d5bc-47c6-ab73-7f4d8a264270',
            },
          },
        ],
      },
      stringTokens: ['petroleum'],
    }

    const raw = {
      blocks: [
        {
          data: {},
          depth: 0,
          inlineStyleRanges: [],
          entityRanges: [
            {
              key: 0,
              offset: 0,
              length: 2,
            },
          ],
          key: 'key',
          text: 'BP petroleum',
          type: 'unstyled',
        },
      ],
      entityMap: {
        0: {
          type: 'in:mention',
          mutability: 'SEGMENTED',
          data: {
            mention: {
              name: 'BP',
              link: '/wiki/topics/321ccdae-d5bc-47c6-ab73-7f4d8a264270',
            },
          },
        },
      },
    }

    expect(rawFromQuery(queryInfo, genKey)).toEqual(raw)
  })
})
