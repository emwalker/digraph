import { RelayRefetchProp } from 'react-relay';
import { EditLinkForm_link as LinkType } from '__generated__/EditLinkForm_link.graphql';
declare type Props = {
    isOpen: boolean;
    link: LinkType;
    relay: RelayRefetchProp;
    toggleForm: () => void;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
