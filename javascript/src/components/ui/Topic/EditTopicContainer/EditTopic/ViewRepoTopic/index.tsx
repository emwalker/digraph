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

  if (synonyms.length === 0) return <Placeholder />

  return (
    <li
      className="Box-row view-repo-topic"
      style={{ borderColor: borderColor(repoTopic.displayColor) }}
    >
      <div>Names and synonyms</div>
      <ul className="Box Box--condensed mt-2" style={{ background: 'inherit' }}>
        {synonyms.map(({ name, locale }, index) => (
          <Synonym key={index} name={name} locale={locale} />
        ))}
      </ul>
    </li >
  )
}
