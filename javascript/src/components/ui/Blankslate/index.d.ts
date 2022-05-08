import { ReactNode } from 'react';
declare type Props = {
    children: ReactNode;
    title?: string | undefined;
};
declare const Blankslate: {
    ({ children, title }: Props): JSX.Element;
    defaultProps: {
        title: null;
    };
};
export default Blankslate;
