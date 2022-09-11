import React, { useCallback, useState } from 'react'
import { graphql, useFragment, useRelayEnvironment } from 'react-relay'
import classNames from 'classnames'

import reviewLinkMutation, { Input } from 'mutations/reviewLinkMutation'
import { Review_link$key } from '__generated__/Review_link.graphql'

type Props = {
  link: Review_link$key,
}

export default function Review(props: Props) {
  const environment = useRelayEnvironment()

  const link = useFragment(
    graphql`
      fragment Review_link on Link {
        displayTitle
        displayUrl
        id

        viewerReview {
          reviewedAt
        }
      }
    `,
    props.link,
  )

  const [reviewed, setReviewed] = useState(false)

  const onChange = useCallback(() => {
    const input: Input = { linkId: link.id, reviewed: !reviewed }
    reviewLinkMutation(environment, input)
    setReviewed(!reviewed)
  }, [setReviewed, reviewLinkMutation, environment])

  const className = classNames(
    'Box-row clearfix Review', 'd-flex', 'flex-items-center', {
      'Review--reviewed': reviewed,
    },
  )

  return (
    <li className={className}>
      <div className="overflow-hidden flex-auto pr-3">
        <div>
          <a className="Link--primary" href={link.displayUrl}>
            { link.displayTitle }
          </a>
        </div>
        <div className="mt-2 link-url branch-name css-truncate css-truncate-target">
          { link.displayUrl }
        </div>
      </div>
      <form className="review-checkbox">
        <div className="form-checkbox">
          <label>
            <input
              checked={reviewed}
              className="input-lg"
              onChange={onChange}
              type="checkbox"
            />
            Reviewed
          </label>
        </div>
      </form>
    </li>
  )
}
