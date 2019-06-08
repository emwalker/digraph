// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import classNames from 'classnames'

import { type Relay } from 'components/types'
import reviewLinkMutation from 'mutations/reviewLinkMutation'
import { type Review_link as Link } from './__generated__/Review_link.graphql'

type Props = {
  link: Link,
  relay: Relay,
}

type State = {
  reviewed: boolean,
}

class Review extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    const { link: { viewerReview } } = props

    this.state = {
      reviewed: viewerReview != null && viewerReview.reviewedAt != null,
    }
  }

  onChange = () => {
    this.setState(
      ({ reviewed }) => {
        reviewLinkMutation(
          this.props.relay.environment,
          [],
          { linkId: this.props.link.id, reviewed: !reviewed },
        )

        return {
          reviewed: !reviewed,
        }
      },
    )
  }

  get className(): string {
    return classNames(
      'Box-row clearfix Review', 'd-flex', 'flex-items-center', {
        'Review--reviewed': this.state.reviewed,
      },
    )
  }

  render = () => {
    const { link: { title, url } } = this.props

    return (
      <li className={this.className}>
        <div className="overflow-hidden flex-auto pr-3">
          <div>
            <a className="link-gray-dark" href={url}>
              { title }
            </a>
          </div>
          <div className="mt-2 link-url branch-name css-truncate css-truncate-target">
            { url }
          </div>
        </div>
        <form>
          <div className="form-checkbox">
            <label>
              <input
                checked={this.state.reviewed}
                className="input-lg"
                onChange={this.onChange}
                type="checkbox"
              />
              Reviewed
            </label>
          </div>
        </form>
      </li>
    )
  }
}

export default createFragmentContainer(Review, {
  link: graphql`
    fragment Review_link on Link {
      id
      title
      url

      viewerReview {
        reviewedAt
      }
    }
  `,
})
