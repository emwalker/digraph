import { ReactNode } from 'react';
declare type Props = {
    children: ReactNode;
    topicName?: string;
    totalCount: number;
};
declare const Container: (props: Props) => JSX.Element;
export default Container;
