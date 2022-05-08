import { Component } from 'react';
declare type Props = {
    className?: string;
    onDelete: () => void;
};
declare class DeleteButton extends Component<Props> {
    static defaultProps: {
        className: string;
    };
    get className(): string;
    confirmAndDelete: () => void;
    render: () => JSX.Element;
}
export default DeleteButton;
