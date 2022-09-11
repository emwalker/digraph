import { RelayRefetchProp } from 'react-relay';
import { EditLinkForm_repoLink$data as RepoLinkType } from '__generated__/EditLinkForm_repoLink.graphql';
import { EditLinkForm_viewer$data as ViewerType } from '__generated__/EditLinkForm_viewer.graphql';
declare type Props = {
    isOpen: boolean;
    repoLink: RepoLinkType;
    relay: RelayRefetchProp;
    viewer: ViewerType;
    toggleForm: () => void;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
