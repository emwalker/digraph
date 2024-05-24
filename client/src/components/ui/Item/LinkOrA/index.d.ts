import React, { ReactNode } from 'react';
import { LocationType } from 'components/types';
type Props = {
    children: ReactNode;
    className: string;
    to: LocationType;
};
declare const LinkOrA: ({ children, className, to }: Props) => React.JSX.Element;
export default LinkOrA;
