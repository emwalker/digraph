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
      <div data-testid="synonym-list">
        {synonyms.map((value, index) => (
          <SortableSynonym key={value.name} id={index} synonym={value} />
        ))}
      </div>
    )
  }

  return (
    <div data-testid="synonym-list">
      <SortableSynonymList
        synonyms={synonyms}
        onDelete={onDelete}
        onUpdate={onUpdate}
      />
    </div>
  )
}
