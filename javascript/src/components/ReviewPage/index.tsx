import React from 'react'
import { graphql, useFragment } from 'react-relay'

import Page from 'components/ui/Page'
import { NodeTypeOf, liftNodes } from 'components/types'
import {
  ReviewPage_view$key,
  ReviewPage_view$data as ViewType,
} from '__generated__/ReviewPage_view.graphql'
import Container from './Container'
import Review from './Review'
import reviewPageQuery, { ViewType as QueryViewType } from './reviewPageQuery'

type RootTopicType = NonNullable<ViewType['topic']>
type LinkType = NodeTypeOf<RootTopicType['childLinks']>

type Props = {
  view: ReviewPage_view$key,
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

const renderReview = (link: LinkType | null) => (
  link && <Review key={link.id} link={link} />
)

const renderNoLinks = () => (
  <div className="blankslate">
    <p>There are no links to review.</p>
  </div>
)

function ReviewPage(props: Props) {
  const view = useFragment(
    graphql`
      fragment ReviewPage_view on View {
        topic(id: $topicId) {
          displayName

          childLinks(first: 100, reviewed: false, descendants: true) {
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
    props.view,
  )

  const topic = view.topic
  const links = liftNodes<LinkType>(topic?.childLinks)
  const totalCount = topic?.childLinks?.totalCount || 0

  return (
    <Container totalCount={totalCount} topicName={topic?.displayName}>
      {links.length > 0
        ? links.map(renderReview)
        : renderNoLinks()}
    </Container>
  )
}

export default ({ view }: Props) => (
  <Page>
    {
      view
        ? <ReviewPage view={view} />
        : <Placeholder />
    }
  </Page>
)
