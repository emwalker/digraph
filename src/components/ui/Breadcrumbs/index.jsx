// @flow
import React, { Component, Fragment, type Node } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import Octicon from 'react-component-octicons'
import { Link } from 'found'
import { pathOr } from 'ramda'

import type { RepositoryType } from 'components/types'

const defaultRepoPath = pathOr(null, ['organization', 'defaultRepository', 'rootTopic', 'resourcePath'])

type Props = {
  orgLogin: string,
  repository: RepositoryType,
}

class Breadcrumbs extends Component<Props> {
  get repoLink(): Node {
    const repoLink = defaultRepoPath(this.props.repository)
    if (!repoLink)
      return this.props.orgLogin

    return (
      <Link to={repoLink}>
        {this.props.orgLogin}
      </Link>
    )
  }

  render = () => {
    const { repository } = this.props

    return (
      <nav aria-label="Breadcrumb" className="mb-1">
        <ol>
          <li className="breadcrumb-item">
            <Octicon name="repo" className="mr-1" />
            {' '}
            {this.repoLink}
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
  }
}

export default createFragmentContainer(Breadcrumbs, graphql`
  fragment Breadcrumbs_repository on Repository {
    displayName
    isPrivate

    organization {
      defaultRepository {
        rootTopic {
          resourcePath
        }
      }
    }
  }
`)
