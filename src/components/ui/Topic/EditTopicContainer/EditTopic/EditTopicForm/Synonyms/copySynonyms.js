// @flow
import { type Synonym } from './types'

export default (source: $ReadOnlyArray<Synonym>): Synonym[] => {
  const dest = []
  source.forEach(({ name, locale }) => {
    const synonym = ({ name, locale }: any)
    dest.push(synonym)
  })
  return dest
}
