/// <reference types="react" />
import { RelayProp } from 'react-relay';
declare type Link = {
    id: string;
};
declare type Props = {
    isOpen: boolean;
    link: Link;
    relay: RelayProp;
    toggleForm: () => void;
};
declare const EditLinkContainer: ({ isOpen, link, relay, toggleForm }: Props) => JSX.Element;
export default EditLinkContainer;
