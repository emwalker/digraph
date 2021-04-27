import { Environment } from 'relay-runtime'
import { commitMutation, GraphQLTaggedNode, Disposable, DeclarativeMutationConfig } from 'react-relay'
import { v1 as uuidv1 } from 'uuid'

import type { Updater } from './types'

type Mutator = (...args: any) => Disposable

function defaultMutation<Input>(mutation: GraphQLTaggedNode, updater?: Updater): Mutator {
  return (
    environment: Environment,
    input: Input,
    config?: DeclarativeMutationConfig,
  ): Disposable => {
    const clientMutationId = uuidv1()

    return commitMutation(
      environment,
      {
        ...config,
        mutation,
        updater,
        variables: {
          input: { ...input, clientMutationId },
        },
      },
    )
  }
}

export default defaultMutation
