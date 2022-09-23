import React, { ReactNode } from 'react'
import { RelayEnvironmentProvider } from 'react-relay/hooks'
import { Props as RelayEnvironmentProviderProps } from 'react-relay/relay-hooks/RelayEnvironmentProvider.react'

interface ProviderProps {
  relayEnvironment: RelayEnvironmentProviderProps['environment'];
  children?: ReactNode;
}

export const Providers = (props: ProviderProps) => {
  const { relayEnvironment, children } = props
  return (
    <RelayEnvironmentProvider environment={relayEnvironment}>
      {children}
    </RelayEnvironmentProvider>
  )
}
