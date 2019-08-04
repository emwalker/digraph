// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import { type CollectionNode } from 'components/types'
import { type ReviewPage_view as View } from './__generated__/ReviewPage_view.graphql'
import Container from './Container'
import Review from './Review'

type Topic = $PropertyType<View, 'topic'>
type Link = CollectionNode<$PropertyType<$NonMaybeType<Topic>, 'links'>>

type Props = {
  view: View,
}

const Placeholder = () => (
  <Container totalCount={0} topicName={null}>
    <div className="blankslate">
      <p>Searching the computers for links to review ...</p>
    </div>
  </Container>
)

class ReviewPage extends Component<Props> {
  get topic(): Topic {
    const { view } = this.props
    return view ? view.topic : null
  }

  get links(): $ReadOnlyArray<?Link> {
    const { topic } = this
    if (!topic) return []

    const { links } = topic
    const edges = links ? links.edges : null
    if (!edges) return []

    return edges.map(edge => edge && edge.node)
  }

  get totalCount(): number {
    const { topic } = this
    if (!topic) return 0

    const { links: { totalCount } } = topic
    return totalCount
  }

  get topicName(): ?string {
    const { topic } = this
    return topic ? topic.displayName : null
  }

  renderReview = (link: ?Link) => link && <Review key={link.id} link={link} />

  renderNoLinks = () => (
    <div className="blankslate">
      <p>There are no links to review.</p>
    </div>
  )

  render = () => {
    const { links } = this

    return (
      <Container totalCount={this.totalCount} topicName={this.topicName}>
        { links.length > 0
          ? links.map(this.renderReview)
          : this.renderNoLinks()
        }
      </Container>
    )
  }
}

export const query = graphql`
query ReviewPage_query_Query(
  $viewerId: ID!,
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
  $topicId: ID!,
) {
  alerts {
    id
    text
    type
  }

  view(
    viewerId: $viewerId,
    currentOrganizationLogin: $orgLogin,
    currentRepositoryName: $repoName,
    repositoryIds: $repoIds,
  ) {
    ...ReviewPage_view
  }
}`

const Wrapper = createFragmentContainer(ReviewPage, {
  view: graphql`
    fragment ReviewPage_view on View {
      topic(id: $topicId) {
        displayName

        links(first: 1000, reviewed: false, descendants: true) {
          totalCount

          edges {
            node {
              id
              ...Review_link
            }
          }
        }
      }
    }
  `,
})

export default ({ props }: { props: Props }) => (
  // eslint-disable-next-line react/prop-types
  props && props.view
    ? <Wrapper {...props} />
    : <Placeholder />
)
