import { Match } from 'found';
import { ViewType as QueryViewType } from './userSettingsQuery';
export declare const query: import("react-relay").GraphQLTaggedNode;
declare type RenderProps = {
    view: QueryViewType;
    match: Match;
};
declare const _default: ({ view, match }: RenderProps) => JSX.Element | null;
export default _default;
