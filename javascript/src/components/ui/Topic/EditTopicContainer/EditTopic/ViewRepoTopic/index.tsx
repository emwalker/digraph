import React from 'react'
import { graphql, useFragment } from 'react-relay'

import Synonym from './ViewRepoTopicSynonym'
import { ViewRepoTopic_repoTopic$key } from '__generated__/ViewRepoTopic_repoTopic.graphql'
import Blankslate from 'components/ui/Blankslate'
import { borderColor } from 'components/helpers'

type Props = {
  repoTopic: ViewRepoTopic_repoTopic$key,
}

const Placeholder = () => (
  <Blankslate>
    <p>There are no details for this repo.</p>
  </Blankslate>
)

const repoTopicFragment = graphql`
  fragment ViewRepoTopic_repoTopic on RepoTopic {
    displayColor
    repoId
    timerangePrefix

    details {
      synonyms {
        name
        locale
      }
    }
  }
`

export default function ViewRepoTopic(props: Props) {
  const repoTopic = useFragment(repoTopicFragment, props.repoTopic)
  const synonyms = repoTopic.details?.synonyms || []
  const timerangePrefix = repoTopic.timerangePrefix

  if (synonyms.length === 0) return <Placeholder />

  return (
    <li
      className="Box-row view-repo-topic"
      data-testid={`repo-topic-${repoTopic.repoId}`}
      style={{ borderColor: borderColor(repoTopic.displayColor) }}
    >
      <div>
        <div>Names and synonyms</div>
        <ul className="Box Box--condensed mt-2" style={{ background: 'inherit' }}>
          {synonyms.map(({ name, locale }, index) => (
            <Synonym key={index} name={name} locale={locale} />
          ))}
        </ul>
      </div>

      <div className="mt-2">
        <div data-testid="timerange">
          <div>Starts at: {timerangePrefix || 'n/a'}</div>
        </div>
      </div>
    </li>
  )
}
