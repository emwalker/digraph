// @flow
import React, { Fragment } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import Octicon from 'react-component-octicons'

type Props = {
  orgLogin: string,
  repository: {
    isPrivate: boolean,
  }
}

const Breadcrumbs = ({ orgLogin, repository }: Props) => (
  <nav aria-label="Breadcrumb" className="mb-1">
    <ol>
      <li className="breadcrumb-item">
        <Octicon name="repo" className="mr-1" />
        {` ${orgLogin}`}
      </li>
      <li
        className="breadcrumb-item breadcrumb-item-selected text-gray"
        aria-current="page"
      >
        {repository.displayName}
        {repository.isPrivate && (
          <Fragment>
            {' '}
            <span className="Label Label--outline v-align-middle">Private</span>
          </Fragment>
        )}
      </li>
    </ol>
  </nav>
)

export default createFragmentContainer(Breadcrumbs, graphql`
  fragment Breadcrumbs_repository on Repository {
    displayName
    isPrivate
  }
`)
