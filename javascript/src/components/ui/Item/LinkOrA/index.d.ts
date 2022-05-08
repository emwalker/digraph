import { ReactNode } from 'react';
import { LocationType } from 'components/types';
declare type Props = {
    children: ReactNode;
    className: string;
    to: LocationType;
};
declare const LinkOrA: ({ children, className, to }: Props) => JSX.Element;
export default LinkOrA;
