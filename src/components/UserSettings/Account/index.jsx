// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Relay, Match } from 'components/types'
import Content from '../Content'
import DeleteAccount from './DeleteAccount'
import { type Account_view as View } from './__generated__/Account_view.graphql'

type Props = {
  match: Match,
  relay: Relay,
  view: View,
}

const Account = ({ match, relay, view }: Props) => {
  const { location: { pathname } } = match

  if (pathname !== '/settings/account') return null

  return (
    <Content>
      <DeleteAccount relay={relay} view={view} />
    </Content>
  )
}

export default createFragmentContainer(Account, {
  view: graphql`
    fragment Account_view on View {
      ...DeleteAccount_view
    }
  `,
})
