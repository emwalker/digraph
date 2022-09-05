/// <reference types="react" />
declare type ItemType = {
    displayName: string;
    id: string;
} | null;
declare type Props = {
    items: ItemType[];
    placeholder: string;
    title: string;
};
declare const _default: ({ items, placeholder, title }: Props) => JSX.Element;
export default _default;
