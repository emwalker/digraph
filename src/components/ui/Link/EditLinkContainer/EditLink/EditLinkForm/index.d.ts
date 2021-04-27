import { RelayRefetchProp } from 'react-relay';
import { EditLinkForm_link as LinkType } from '__generated__/EditLinkForm_link.graphql';
declare type Props = {
    isOpen: boolean;
    link: LinkType;
    orgLogin: string;
    relay: RelayRefetchProp;
    toggleForm: () => void;
};
declare const _default: import("react-relay").Container<Props>;
export default _default;
