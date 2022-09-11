import React, { ReactNode, useCallback } from 'react'
import { GoTrashcan } from 'react-icons/go'
import { graphql, useFragment } from 'react-relay'

import { Synonym_synonym$key as SynonymType } from '__generated__/Synonym_synonym.graphql'

type Props = {
  dragHandle?: ReactNode,
  onDelete?: (index: number) => void,
  position?: number,
  synonym: SynonymType,
}

export default function Synonym(props: Props) {
  const data = useFragment(
    graphql`
      fragment Synonym_synonym on Synonym {
        name
        locale
      }
    `,
    props.synonym,
  )

  const onClick = useCallback(() => {
    if (!props.onDelete || !props.position) return
    props.onDelete(props.position)
  }, [props.onDelete])

  return (
    <li className="Box-row clearfix css-truncate p-2 d-flex">
      { props.dragHandle }
      <div className="col-10">{data.name}</div>
      <div className="col-1">
        { data.locale }
      </div>
      <div className="col-1 remove-synonym">
        { props.onDelete && (
            <span tabIndex={0} role="button" onClick={onClick}>
              <GoTrashcan />
            </span>
        )
        }
      </div>
    </li>
  )
}