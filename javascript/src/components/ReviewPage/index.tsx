import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import Page from 'components/ui/Page'
import { NodeTypeOf, liftNodes } from 'components/types'
import { ReviewPage_view as ViewType } from '__generated__/ReviewPage_view.graphql'
import Container from './Container'
import Review from './Review'
import reviewPageQuery, { ViewType as QueryViewType } from './reviewPageQuery'

type RootTopicType = NonNullable<ViewType['topic']>
type LinkType = NodeTypeOf<RootTopicType['links']>

type Props = {
  view: ViewType,
}

export const query = reviewPageQuery
export type ContainerViewType = QueryViewType

const Placeholder = () => (
  <Container totalCount={0}>
    <div className="blankslate">
      <p>Searching the computers for links to review ...</p>
    </div>
  </Container>
)

class ReviewPage extends Component<Props> {
  get topic() {
    return this.props.view.topic
  }

  get links() {
    return liftNodes<LinkType>(this.topic?.links)
  }

  get totalCount(): number {
    const { topic } = this
    if (!topic) return 0

    const { links: { totalCount } } = topic
    return totalCount
  }

  get topicName() {
    return this.topic?.displayName
  }

  renderReview = (link: LinkType | null) => link && <Review key={link.id} link={link} />

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
          : this.renderNoLinks()}
      </Container>
    )
  }
}

const Wrapper = createFragmentContainer(ReviewPage, {
  view: graphql`
    fragment ReviewPage_view on View {
      topic(path: $topicPath) {
        displayName

        links(first: 100, reviewed: false, descendants: true) {
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

type RenderProps = {
  view: QueryViewType,
}

export default ({ view }: RenderProps) => (
  <Page>
    {
      view
        ? <Wrapper view={view} />
        : <Placeholder />
    }
  </Page>
)
