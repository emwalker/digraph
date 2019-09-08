// @flow
import React from 'react'

import type { Match } from 'components/types'
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
        If you run into difficulties, send an email to the
        {' '}
        <a href="mailto:eric.walker+digraph-support">app maintainer</a>
        {' '}
        with your question, and he&apos;ll look into it.
      </p>
    </Content>
  )
}

export default Support
