import { RelayProp } from 'react-relay';
import { Review_link as Link } from '__generated__/Review_link.graphql';
declare type Props = {
    link: Link;
    relay: RelayProp;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
