import { Component, ReactNode } from 'react';
declare type Props = {
    children: ReactNode;
    topicName?: string;
};
declare class Container extends Component<Props> {
    get title(): string;
    render: () => JSX.Element;
}
export default Container;
