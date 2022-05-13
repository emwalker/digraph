/// <reference types="react" />
import { RelayProp } from 'react-relay';
declare type Link = {
    id: string;
};
declare type Props = {
    isOpen: boolean;
    link: Link;
    orgLogin: string;
    relay: RelayProp;
    toggleForm: () => void;
};
declare const EditLinkContainer: ({ isOpen, link, orgLogin, relay, toggleForm }: Props) => JSX.Element;
export default EditLinkContainer;
