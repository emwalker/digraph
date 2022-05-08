import { EditorState, convertFromRaw } from 'draft-js'

import queryFromState from './queryFromState'

const stateFor = (raw: any) => EditorState.createWithContent(convertFromRaw(raw))
const queryFor = (raw: any) => queryFromState(stateFor(raw))
const toString = (raw: any): string => queryFor(raw).toString()

describe('queryFromState', () => {
  it('handles simple text', () => {
    const raw = {
      blocks: [
        {
          text: 'one two three',
          entityRanges: [],
        },
      ],
      entityMap: {},
    }

    expect(toString(raw)).toEqual('one two three')
  })

  it('handles embedded topics', () => {
    const raw = {
      blocks: [
        {
          text: 'one in:Germany two',
          entityRanges: [
            { offset: 4, length: 10, key: 0 },
          ],
        },
      ],
      entityMap: {
        0: {
          data: {
            mention: {
              link: '/wiki/topics/1',
            },
          },
        },
      },
    }

    expect(toString(raw)).toEqual('one in:/wiki/topics/1 two')
  })
})
