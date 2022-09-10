import { RelayRefetchProp } from 'react-relay';
import { EditLinkForm_repoLink$data as RepoLinkType } from '__generated__/EditLinkForm_repoLink.graphql';
declare type Props = {
    isOpen: boolean;
    repoLink: RepoLinkType;
    relay: RelayRefetchProp;
    toggleForm: () => void;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
