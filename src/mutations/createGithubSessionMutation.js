// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'
import type {
  CreateGithubSessionInput,
  createGithubSessionMutationResponse,
} from './__generated__/createGithubSessionMutation.graphql'

export type Input = CreateGithubSessionInput
export type Response = createGithubSessionMutationResponse

export default defaultMutation(graphql`
  mutation createGithubSessionMutation(
    $input: CreateGithubSessionInput!
  ) {
    createGithubSession(input: $input) {
      alerts {
        id
        text
        type
      }

      userEdge {
        node {
          id
        }
      }

      sessionEdge {
        node {
          id
        }
      }
    }
  }
`, flashMessageUpdater('createGithubSession'))
