import React, { ReactNode } from 'react';
import { Props as RelayEnvironmentProviderProps } from 'react-relay/relay-hooks/RelayEnvironmentProvider.react';
interface ProviderProps {
    relayEnvironment: RelayEnvironmentProviderProps['environment'];
    children?: ReactNode;
}
export declare const Providers: (props: ProviderProps) => React.JSX.Element;
export {};
