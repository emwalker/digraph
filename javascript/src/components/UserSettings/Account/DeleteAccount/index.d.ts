import { RelayProp } from 'react-relay';
import { DeleteAccount_view$data as ViewType } from '__generated__/DeleteAccount_view.graphql';
declare type Props = {
    relay: RelayProp;
    view: ViewType | undefined;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
