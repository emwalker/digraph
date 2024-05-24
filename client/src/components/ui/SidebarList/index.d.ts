import React from 'react';
type ItemType = {
    displayName: string;
    id: string;
} | null;
type Props = {
    items: ItemType[];
    placeholder: string;
    title: string;
};
declare const _default: ({ items, placeholder, title }: Props) => React.JSX.Element;
export default _default;
