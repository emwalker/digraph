import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Match } from 'found'

import { Account_view$data as ViewType } from '__generated__/Account_view.graphql'
import Content from '../Content'
import DeleteAccount from './DeleteAccount'

type Props = {
  match: Match,
  view: ViewType | undefined,
}

const Account = ({ match, view }: Props) => {
  const { location: { pathname } } = match

  if (pathname !== '/settings/account') return null

  return (
    <Content>
      <DeleteAccount
        // @ts-expect-error
        view={view}
      />
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
