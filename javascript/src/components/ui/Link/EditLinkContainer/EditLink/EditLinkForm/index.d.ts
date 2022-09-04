import { RelayRefetchProp } from 'react-relay';
import { EditLinkForm_linkDetails as LinkDetailsType } from '__generated__/EditLinkForm_linkDetails.graphql';
declare type Props = {
    isOpen: boolean;
    linkDetails: LinkDetailsType;
    relay: RelayRefetchProp;
    toggleForm: () => void;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
