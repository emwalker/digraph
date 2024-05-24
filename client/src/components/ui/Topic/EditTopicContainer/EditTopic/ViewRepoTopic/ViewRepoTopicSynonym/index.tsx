import React from 'react'

type Props = {
  name: string,
  locale: string,
}

export default function ViewRepoTopicSynonym(props: Props) {
  return (
    <li className="Box-row clearfix css-truncate p-2 d-flex">
      <div className="col-10">{props.name}</div>
      <div className="col-1">
        { props.locale }
      </div>
    </li>
  )
}
