// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import DocumentTitle from 'react-document-title'

import { type CollectionNode } from 'components/types'
import { type ReviewPage_view as View } from './__generated__/ReviewPage_view.graphql'
import Container from './Container'
import Review from './Review'

type Link = CollectionNode<$PropertyType<View, 'links'>>

type Props = {
  view: View,
}

const Placeholder = () => <div>Loading ...</div>

class ReviewPage extends Component<Props> {
  get links(): $ReadOnlyArray<?Link> {
    const { view: { links: { edges } } } = this.props
    if (!edges) return []
    return edges.map(edge => edge && edge.node)
  }

  renderReview = (link: ?Link) => link && <Review key={link.id} link={link} />

  render = () => {
    const { links } = this

    return (
      <DocumentTitle title="Links for review">
        <Container>
          { links.length > 0
            ? (
              <div className="Box Box--condensed">
                <ul>
                  { links.map(this.renderReview) }
                </ul>
              </div>
            ) : (
              <div>There are no links to review.</div>
            )
          }
        </Container>
      </DocumentTitle>
    )
  }
}

export const query = graphql`
query ReviewPage_query_Query(
  $orgLogin: String!,
  $repoName: String,
  $repoIds: [ID!],
) {
  alerts {
    id
    text
    type
  }

  view(
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
      links(first: 1000, reviewed: false) {
        edges {
          node {
            id
            ...Review_link
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
