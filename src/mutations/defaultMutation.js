import { commitMutation } from 'react-relay'
import uuidv1 from 'uuid/v1'

export default mutation => (environment, configs, input) => {
  const clientMutationId = uuidv1()

  return commitMutation(
    environment,
    {
      mutation,
      configs,
      variables: {
        input: { clientMutationId, ...input },
      },
    },
  )
}
