import { FormEvent } from 'react';
declare type Props = {
    className: string;
    disabled?: boolean;
    id: string;
    label: string;
    onChange?: (event: FormEvent<HTMLInputElement>) => void;
    placeholder?: string;
    value: string | undefined;
};
declare const _default: ({ className, disabled, id, label, onChange, placeholder, value }: Props) => JSX.Element;
export default _default;
