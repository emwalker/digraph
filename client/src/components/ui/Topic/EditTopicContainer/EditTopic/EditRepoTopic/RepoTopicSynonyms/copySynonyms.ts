import { SynonymType } from 'components/types'

export default (source: readonly SynonymType[]): SynonymType[] => {
  const dest: SynonymType[] = []
  source.forEach(({ name, locale }) => {
    const synonym = {
      name,
      locale,
    }
    dest.push(synonym)
  })
  return dest
}
