import React, { Component, ReactNode } from 'react';
declare type Props = {
    children: ReactNode;
};
declare type State = {
    hasError: boolean;
};
declare class ErrorBoundary extends Component<Props, State> {
    static getDerivedStateFromError(): {
        hasError: boolean;
    };
    constructor(props: Props);
    componentDidCatch(error: Error, info: Object): void;
    render: () => string | number | boolean | React.ReactFragment | JSX.Element | null | undefined;
}
export default ErrorBoundary;
