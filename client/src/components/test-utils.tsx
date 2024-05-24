import React, { ReactElement } from 'react'
import userEvent from '@testing-library/user-event'
import { render, RenderOptions } from '@testing-library/react'
import { createMockEnvironment, MockEnvironment } from 'relay-test-utils'
import { Providers } from './Providers'
import { Environment } from 'react-relay'

const AllProviders = ({
  children,
  relayEnvironment,
}: {
  children?: React.ReactNode;
  relayEnvironment: Environment;
}) => {
  return (
    <React.Suspense fallback="Loading...">
      <Providers relayEnvironment={relayEnvironment}>{children}</Providers>
    </React.Suspense>
  )
}

function customRender(ui: ReactElement, options?: RenderOptions): { environment: MockEnvironment } {
  const environment = createMockEnvironment()
  render(ui, {
    wrapper: (props) => (
      <AllProviders relayEnvironment={environment} {...props} />
    ),
    ...options,
  })
  return { environment }
}

function renderWithUser(component: ReactElement) {
  return {
    user: userEvent.setup(),
    ...customRender(component),
  }
}

export { customRender as render, renderWithUser }
