import { Component } from 'react';
import { AlertType } from 'components/types';
declare type AlertPayload = {
    id: string;
    type: AlertType;
    text: string;
};
declare type Props = {
    initialAlerts: readonly AlertPayload[];
};
declare type State = {
    messages: readonly AlertPayload[];
};
declare class FlashMessages extends Component<Props, State> {
    constructor(props: Props);
    get alerts(): JSX.Element[];
    removeMessage: (message: AlertPayload) => void;
    addMessage: (message: AlertPayload) => void;
    render: () => JSX.Element | null;
}
export default FlashMessages;
