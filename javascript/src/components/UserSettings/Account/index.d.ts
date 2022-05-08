import { Match } from 'found';
import { Account_view as ViewType } from '__generated__/Account_view.graphql';
declare type Props = {
    match: Match;
    view: ViewType | undefined;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
