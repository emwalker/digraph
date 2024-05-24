import React, { Component, ReactElement } from 'react';
import { AlertMessageType } from 'components/types';
import Alert from './Alert';
type Props = {
    initialAlertMessages: readonly AlertMessageType[];
};
type State = {
    messages: readonly AlertMessageType[];
    alerts: ReactElement<typeof Alert>[];
};
declare class FlashMessages extends Component<Props, State> {
    constructor(props: Props);
    get alerts(): React.JSX.Element[];
    removeAlert: (component: ReactElement<typeof Alert>) => void;
    removeMessage: (alert: AlertMessageType) => void;
    addMessage: (message: AlertMessageType) => void;
    addAlert: (alert: ReactElement<typeof Alert>) => void;
    render: () => React.JSX.Element | null;
}
export default FlashMessages;
