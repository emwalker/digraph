/// <reference types="react" />
declare type ItemType = {
    display: string;
    path: string;
} | null;
declare type Props = {
    items: ItemType[];
    orgLogin: string;
    placeholder: string;
    repoName: string;
    title: string;
};
declare const _default: ({ items, orgLogin, placeholder, repoName, title }: Props) => JSX.Element;
export default _default;
