// @flow
import React, { Component } from 'react'
import { createRefetchContainer, graphql } from 'react-relay'

import type { LinkType, Relay, UserType, ViewType } from 'components/types'
import { liftNodes } from 'utils'
import EditLinkForm from './EditLinkForm'

type Props = {
  isOpen: boolean,
  orgLogin: string,
  toggleForm: Function,
}

type PropsType = {
  error: ?Object,
  orgLogin: string,
  relay: Relay,
  link: LinkType,
  view: ViewType,
  viewer: UserType,
} & Props

type RenderProps = {
  error: ?Object,
  props: ?PropsType,
}

/* eslint react/prop-types: 0 */
/* eslint react/no-unused-prop-types: 0 */

class EditLink extends Component<PropsType> {
  componentDidMount = () => {
    const { refetch } = this.props.relay
    if (refetch) {
      setTimeout(() => {
        refetch({
          orgLogin: this.props.orgLogin,
          count: 3000,
        })
      }, 100)
    }
  }

  render = () => {
    const { error, isOpen, orgLogin, relay, toggleForm, link, viewer } = this.props

    if (error)
      return <div>{error.message}</div>

    if (!link)
      return null

    return (
      <EditLinkForm
        availableTopics={liftNodes(link.availableTopics)}
        isOpen={isOpen}
        orgLogin={orgLogin}
        relay={relay}
        selectedTopics={liftNodes(link.selectedTopics)}
        toggleForm={toggleForm}
        link={link}
        viewer={viewer}
      />
    )
  }
}

const Wrapped = createRefetchContainer(EditLink, graphql`
  fragment EditLink_link on Link @argumentDefinitions(
    count: {type: "Int!", defaultValue: 10}
  ) {
    selectedTopics: parentTopics(first: 3000) {
      edges {
        node {
          id
          name
        }
      }
    }

    availableTopics: availableParentTopics(first: $count) {
      edges {
        node {
          id
          name
        }
      }
    }

    ...EditLinkForm_link
  }
`, graphql`
  query EditLinkRefetchQuery(
    $orgLogin: String!,
    $repoName: String,
    $repoIds: [ID!],
    $linkId: ID!,
    $count: Int!,
  ) {
    view(
      currentOrganizationLogin: $orgLogin,
      currentRepositoryName: $repoName,
      repositoryIds: $repoIds,
    ) {
      link(id: $linkId) {
        ...EditLink_link @arguments(count: $count)
      }
    }
  }
`)

export default ({ isOpen, orgLogin, toggleForm }: Props) => ({ error, props }: RenderProps) => {
  if (!props)
    return null

  const { view, viewer } = props

  return (
    <Wrapped
      error={error}
      isOpen={isOpen}
      orgLogin={orgLogin}
      relay={props.relay}
      toggleForm={toggleForm}
      link={view.link}
      view={view}
      viewer={viewer}
    />
  )
}
