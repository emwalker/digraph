import React, { useCallback } from 'react'
import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { GoThreeBars, GoTrashcan } from 'react-icons/go'

import { SynonymType } from 'components/types'

type Props = {
  id: number,
  synonym: SynonymType,
  onDelete?: ((position: number) => void) | null,
}

const DragHandle = (props: any) => (
  <GoThreeBars
    className="synonym-drag-handle"
    style={{ width: '20px', height: '17px', color: '#d1d5da' }}
    {...props}
  />
)

export default function SortableItem({ id, synonym, onDelete }: Props) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
  } = useSortable({ id })
  const style = { transform: CSS.Transform.toString(transform) }

  const onClick = useCallback(() => {
    if (!onDelete || !id) return
    onDelete(id)
  }, [onDelete, id])

  return (
    <li
      ref={setNodeRef}
      style={style}
      className="Box-row clearfix css-truncate p-2 d-flex"
    >
      <DragHandle {...listeners} {...attributes} />

      <div className="col-10">{synonym.name}</div>
      <div className="col-1">
        { synonym.locale }
      </div>

       <div className="col-1 remove-synonym">
        {onDelete && (
          <span tabIndex={0} role="button" onClick={onClick}>
            <GoTrashcan />
          </span>
        )}
      </div>
    </li>
  )
}
