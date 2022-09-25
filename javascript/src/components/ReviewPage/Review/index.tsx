import React, { useCallback, useState } from 'react'
import { graphql, useFragment, useMutation } from 'react-relay'
import classNames from 'classnames'

import reviewLinkQuery from 'mutations/reviewLinkMutation'
import { reviewLinkMutation } from '__generated__/reviewLinkMutation.graphql'
import { Review_link$key } from '__generated__/Review_link.graphql'

type Props = {
  link: Review_link$key,
}

const linkFragment = graphql`
  fragment Review_link on Link {
    displayTitle
    displayUrl
    id

    viewerReview {
      reviewedAt
    }
  }
`

export default function Review(props: Props) {
  const reviewLink = useMutation<reviewLinkMutation>(reviewLinkQuery)[0]
  const link = useFragment(linkFragment, props.link)
  const [reviewed, setReviewed] = useState(false)

  const onChange = useCallback(() => {
    reviewLink({
      variables: {
        input: { linkId: link.id, reviewed: !reviewed },
      },
    })
    setReviewed(!reviewed)
  }, [setReviewed, reviewLink])

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
