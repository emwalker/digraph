// @flow
import React, { Component, Fragment, type Node } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import Octicon from 'react-component-octicons'
import { Link } from 'found'
import { pathOr } from 'ramda'

import type { RepositoryType } from 'components/types'

const orgFrom = pathOr(null, ['organization'])

type Props = {
  orgLogin: string,
  repository: RepositoryType,
}

class Breadcrumbs extends Component<Props> {
  get repoLink(): Node {
    const org = orgFrom(this.props.repository)
    if (!org)
      return this.props.orgLogin

    const { defaultRepository: repo } = org
    const { rootTopic: topic } = repo

    const to = {
      pathname: topic.resourcePath,
      state: {
        orgLogin: org.login,
        repoName: repo.displayName,
        itemTitle: topic.name,
      },
    }

    return (
      <Link to={to}>
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
      login

      defaultRepository {
        displayName

        rootTopic {
          name
          resourcePath
        }
      }
    }
  }
`)
