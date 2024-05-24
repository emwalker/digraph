import React, { Component, ReactNode } from 'react';
type Props = {
    children: ReactNode;
};
type State = {
    hasError: boolean;
};
declare class ErrorBoundary extends Component<Props, State> {
    static getDerivedStateFromError(): {
        hasError: boolean;
    };
    constructor(props: Props);
    componentDidCatch(error: Error, info: Object): void;
    render: () => string | number | boolean | Iterable<React.ReactNode> | React.JSX.Element | null | undefined;
}
export default ErrorBoundary;
