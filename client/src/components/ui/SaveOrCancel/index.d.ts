import React, { MouseEventHandler } from 'react';
type Props = {
    onSave: MouseEventHandler<HTMLButtonElement>;
    onCancel: () => void;
};
declare const _default: ({ onSave, onCancel }: Props) => React.JSX.Element;
export default _default;
