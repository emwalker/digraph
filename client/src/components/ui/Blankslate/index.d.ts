import React, { ReactNode } from 'react';
type Props = {
    children: ReactNode;
    title?: string | undefined;
};
declare const Blankslate: {
    ({ children, title }: Props): React.JSX.Element;
    defaultProps: {
        title: null;
    };
};
export default Blankslate;
