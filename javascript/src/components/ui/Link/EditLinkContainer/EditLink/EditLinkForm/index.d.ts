import { RelayRefetchProp } from 'react-relay';
import { EditLinkForm_linkDetail as LinkDetailType } from '__generated__/EditLinkForm_linkDetail.graphql';
declare type Props = {
    isOpen: boolean;
    linkDetail: LinkDetailType;
    relay: RelayRefetchProp;
    toggleForm: () => void;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
