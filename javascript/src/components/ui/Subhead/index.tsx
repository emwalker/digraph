import React from 'react'

import useDocumentTitle from 'utils/useDocumentTitle'

type Props = {
  heading: string,
  renderHeadingDetail?: Function,
}

const Subhead = (props: Props) => {
  const { heading, renderHeadingDetail } = props

  useDocumentTitle(`${heading} | Digraph`)

  return (
    <div className="Subhead gutter">
      <div className="Subhead-heading col-lg-12 col-12">
        <div>{ heading }</div>
        { renderHeadingDetail && renderHeadingDetail() }
      </div>
    </div>
  )
}

Subhead.defaultProps = {
  renderHeadingDetail: null,
}

export default Subhead
