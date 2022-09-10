import { graphql } from 'react-relay'

import {
  CreateGithubSessionInput,
  createGithubSessionMutation$data,
} from '__generated__/createGithubSessionMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = CreateGithubSessionInput
export type Response = createGithubSessionMutation$data

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
