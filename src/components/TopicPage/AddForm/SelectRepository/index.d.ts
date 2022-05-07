import { RelayProp } from 'react-relay';
import { SelectRepository_viewer as ViewerType } from '__generated__/SelectRepository_viewer.graphql';
declare type Props = {
    relay: RelayProp;
    viewer: ViewerType;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
