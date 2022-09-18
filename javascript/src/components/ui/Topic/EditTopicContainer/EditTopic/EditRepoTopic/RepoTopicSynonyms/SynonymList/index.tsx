import React from 'react'

import { SynonymType } from 'components/types'
import SortableSynonym from './SortableSynonym'
import SortableSynonymList from './SortableSynonymList'

type Props = {
  canUpdate: boolean,
  onDelete: (position: number) => void,
  onUpdate: (synonyms: SynonymType[]) => void,
  synonyms: readonly SynonymType[],
}

export default function SynonymList({ canUpdate, onDelete, onUpdate, synonyms }: Props) {
  const canSort = canUpdate && synonyms.length > 1

  if (!canSort) {
    return (
      <>
        {synonyms.map((value, index) => (
          <SortableSynonym key={value.name} id={index} synonym={value} />
        ))}
      </>
    )
  }

  return (
    <SortableSynonymList
      synonyms={synonyms}
      onDelete={onDelete}
      onUpdate={onUpdate}
    />
  )
}
