import React, { Component } from 'react';
type Props = {
    className?: string;
    onDelete: () => void;
};
declare class DeleteButton extends Component<Props> {
    static defaultProps: {
        className: string;
    };
    get className(): string;
    confirmAndDelete: () => void;
    render: () => React.JSX.Element;
}
export default DeleteButton;
