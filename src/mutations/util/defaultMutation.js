import { commitMutation } from 'react-relay'
import uuidv1 from 'uuid/v1'

export default (mutation, updater) => (environment, connectionConfigs, input, config) => {
  const clientMutationId = uuidv1()

  return commitMutation(
    environment,
    {
      ...config,
      mutation,
      configs: connectionConfigs,
      updater,
      variables: {
        input: { clientMutationId, ...input },
      },
    },
  )
}
