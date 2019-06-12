// @flow
import type { CollectionNode } from 'components/types'
import type { Synonyms_topic } from './__generated__/Synonyms_topic.graphql'

export type Topic = Synonyms_topic
export type Synonyms = $PropertyType<Topic, 'synonyms'>
export type Synonym = $NonMaybeType<$ElementType<Synonyms, number>>
