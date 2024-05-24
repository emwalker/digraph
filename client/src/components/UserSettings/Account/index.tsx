import React from 'react'
import { graphql, useFragment } from 'react-relay'
import { Match } from 'found'

import { Account_view$key } from '__generated__/Account_view.graphql'
import Content from '../Content'
import DeleteAccount from './DeleteAccount'

type Props = {
  match: Match,
  view: Account_view$key,
}

export default function Account(props: Props) {
  const view = useFragment(
    graphql`
      fragment Account_view on View {
        ...DeleteAccount_view
      }
    `,
    props.view,
  )

  const pathname = props.match?.location?.pathname
  if (pathname !== '/settings/account') return null

  return (
    <Content>
      {view && <DeleteAccount view={view} />}
    </Content>
  )
}
