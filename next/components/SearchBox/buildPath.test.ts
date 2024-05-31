import { describe, expect, test } from '@jest/globals'
import { buildPath } from './buildPath'
import { ROOT_TOPIC_ID } from '@/lib/constants'
import { Topic, QueryInfo } from '@/lib/__generated__/graphql'

const queryInfo =
  /* @ts-expect-error */
  (topics: Partial<Topic>[], phrases: string[]): QueryInfo => ({ topics, phrases })

describe('buildPath', () => {
  test('empty input', () => {
    expect(buildPath([], queryInfo([], []), new Map())).toBe(`/topics/${ROOT_TOPIC_ID}`)
  })

  test('a newly selected topic', () => {
    const info = queryInfo([], [])
    expect(buildPath(['Biology'], info, new Map([['Biology', '1']]))).toBe('/topics/1')
  })

  test('a search term', () => {
    const info = queryInfo([], [])
    expect(buildPath(['Biology', 'water'], info, new Map([['Biology', '1']])))
      .toBe('/topics/1/q/water')
  })

  test('two new topics', () => {
    const info = queryInfo([], [])
    expect(buildPath(['Biology', 'Water'], info, new Map([['Biology', '1'], ['Water', '2']])))
      .toBe('/topics/1/q/in:2')
  })

  test('two existing topics', () => {
    const info = queryInfo(
      [{ id: '1', displayName: 'Biology' }, { id: '2', displayName: 'Water' }],
      []
    )
    expect(buildPath(['Biology', 'Water'], info, new Map())).toBe('/topics/1/q/in:2')
  })

  test('three existing topics', () => {
    const info = queryInfo(
      [
        { id: '1', displayName: 'Biology' },
        { id: '2', displayName: 'Water' },
        { id: '3', displayName: 'Chemistry' },
      ],
      []
    )
    expect(buildPath(['Biology', 'Water', 'Chemistry'], info, new Map()))
      .toBe('/topics/1/q/in:2 in:3')
  })

  test('two existing topics and a new topic', () => {
    const info = queryInfo(
      [
        { id: '1', displayName: 'Biology' },
        { id: '2', displayName: 'Water' },
      ],
      []
    )
    expect(buildPath(['Biology', 'Water', 'Chemistry'], info, new Map([['Chemistry', '3']])))
      .toBe('/topics/1/q/in:2 in:3')
  })
})
