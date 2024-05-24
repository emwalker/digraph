import React, { MouseEventHandler, type ReactNode } from 'react';
import { AlertMessageType } from 'components/types';
export type OnCloseType = MouseEventHandler<HTMLButtonElement>;
type Props = {
    children?: ReactNode;
    alert: AlertMessageType;
    onClose?: OnCloseType;
};
export default function Alert({ children, alert, onClose }: Props): React.JSX.Element;
export {};
