import { Component } from 'react';
import { AlertType } from 'components/types';
declare type Props = {
    message: {
        text: string;
        type: AlertType;
    };
    onClose: () => void;
};
declare class Alert extends Component<Props> {
    get className(): string;
    alertClass: (type: AlertType) => string;
    render: () => JSX.Element;
}
export default Alert;
