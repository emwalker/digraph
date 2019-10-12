// @flow
import { Environment } from 'relay-runtime'
import { commitMutation } from 'react-relay'
import uuidv1 from 'uuid/v1'

import type { Updater } from './types'

export type Config<R> = {|
  optimisticResponse?: Object,
  configs?: Array<*>,
  onCompleted?: (R) => void,
  onError?: (error: Object) => void,
|}

function defaultMutation<Mutation: {}>(mutation: Mutation, updater?: Updater) {
  return function mutator<Input: {}, Result: {}>(
    environment: Environment,
    input: Input,
    config?: Config<Result>,
  ) {
    const clientMutationId = uuidv1()

    return commitMutation(
      environment,
      {
        ...config,
        mutation,
        updater,
        variables: {
          input: { clientMutationId, ...input },
        },
      },
    )
  }
}

export default defaultMutation
