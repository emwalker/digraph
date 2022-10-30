import { MouseEventHandler, type ReactNode } from 'react';
import { AlertMessageType } from 'components/types';
export declare type OnCloseType = MouseEventHandler<HTMLButtonElement>;
declare type Props = {
    children?: ReactNode;
    alert: AlertMessageType;
    onClose?: OnCloseType;
};
export default function Alert({ children, alert, onClose }: Props): JSX.Element;
export {};
