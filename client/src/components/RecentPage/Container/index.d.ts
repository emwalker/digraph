import React, { Component, ReactNode } from 'react';
type Props = {
    children: ReactNode;
    title?: string;
};
declare class Container extends Component<Props> {
    get title(): string;
    render: () => React.JSX.Element;
}
export default Container;
