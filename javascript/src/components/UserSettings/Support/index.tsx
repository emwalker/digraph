import React from 'react'
import { Match } from 'found'

import Content from '../Content'

type Props = {
  match: Match,
}

const Support = ({ match }: Props) => {
  if (match.location.pathname !== '/settings/support') return null

  return (
    <Content>
      <div className="Subhead">
        <div className="Subhead-heading">Getting help</div>
      </div>

      <p>
        Have a problem or feedback to give? Send an email with your comments or questions to
        eric.walker@gmail.com.
      </p>
    </Content>
  )
}

export default Support
