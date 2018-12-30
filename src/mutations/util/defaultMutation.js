import { commitMutation } from 'react-relay'
import uuidv1 from 'uuid/v1'

export default (mutation, updater) => (environment, configs, input) => {
  const clientMutationId = uuidv1()

  return commitMutation(
    environment,
    {
      mutation,
      configs,
      updater,
      variables: {
        input: { clientMutationId, ...input },
      },
    },
  )
}
