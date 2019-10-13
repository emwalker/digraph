// @flow
import React, { Component, type Node } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { GoRepo } from 'react-icons/go'
import { Link } from 'found'
import { pathOr } from 'ramda'

import type { Breadcrumbs_repository as Repository } from './__generated__/Breadcrumbs_repository.graphql'

const orgFrom = pathOr(null, ['organization'])

type Props = {
  orgLogin: string,
  repository: Repository,
}

class Breadcrumbs extends Component<Props> {
  get repoLink(): Node {
    const org = orgFrom(this.props.repository)
    if (!org) return this.props.orgLogin

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
            <GoRepo className="mr-1" />
            {' '}
            {this.repoLink}
          </li>
          <li
            className="breadcrumb-item breadcrumb-item-selected text-gray"
            aria-current="page"
          >
            {repository.displayName}
            {repository.isPrivate && (
              <>
                {' '}
                <span className="Label Label--outline v-align-middle">Private</span>
              </>
            )}
          </li>
        </ol>
      </nav>
    )
  }
}

export default createFragmentContainer(Breadcrumbs, {
  repository: graphql`
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
  `,
})
