// @flow

type Response = {
  data: ?{
    alerts: ?Object[],
  },
  errors: ?Object[],
}

type Operation = {
  text: string,
}

// Workaround for Relay Modern weirdness
// - https://github.com/facebook/relay/issues/1913
// - https://github.com/facebook/relay/issues/1913#issuecomment-358636018
let alertId = 0

const addAlerts = (response: Response) => {
  if (!response || !response.errors || response.errors.length < 1) return response

  const alerts = []
  response.errors.forEach((error) => {
    alertId += 1

    alerts.push({
      id: `client:alert:${alertId}`,
      text: error.message,
      type: 'ERROR',
    })
  })

  if (response.data) response.data.alerts = alerts
  return response
}

export default (operation: Operation, variables: Object): Promise<Object> => (
  fetch('/graphql', {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
    },
    body: JSON.stringify({
      query: operation.text,
      variables,
    }),
  }).then(response => response.json()).then(addAlerts)
)
