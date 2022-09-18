import React, { useCallback } from 'react'
import {
  DndContext, closestCenter, KeyboardSensor, PointerSensor, useSensor, useSensors, DragEndEvent,
} from '@dnd-kit/core'
import {
  arrayMove, SortableContext, sortableKeyboardCoordinates, verticalListSortingStrategy,
} from '@dnd-kit/sortable'

import { SynonymType } from 'components/types'
import SortableSynonym from '../SortableSynonym'

type Props = {
  synonyms: readonly SynonymType[],
  onDelete: ((position: number) => void) | null,
  onUpdate: (synonyms: SynonymType[]) => void,
}

export default function SortableSynonymList({ synonyms, onUpdate, onDelete }: Props) {
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    }),
  )

  const sortedIds = synonyms.map((_, index) => index)

  const handleDragEnd = useCallback((event: DragEndEvent) => {
    const { active, over } = event

    if (!over) return
    if (active.id === over.id) return

    const oldIndex = sortedIds.indexOf(active.id as number)
    const newIndex = sortedIds.indexOf(over.id as number)
    const newIds = arrayMove(sortedIds, oldIndex, newIndex)
    onUpdate(newIds.map((id) => synonyms[id]))
  }, [sortedIds, synonyms, onUpdate, arrayMove])

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      onDragEnd={handleDragEnd}
    >
      <SortableContext
        items={sortedIds}
        strategy={verticalListSortingStrategy}
      >
        {sortedIds.map((id) => (
          <SortableSynonym key={id} id={id} synonym={synonyms[id]} onDelete={onDelete} />
        ))}
      </SortableContext>
    </DndContext>
  )
}
