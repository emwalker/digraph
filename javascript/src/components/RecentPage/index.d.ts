/// <reference types="react" />
import { RecentPage_recent_QueryResponse as Response } from '__generated__/RecentPage_recent_Query.graphql';
declare type ViewType = Response['view'];
declare type Props = {
    view: ViewType;
};
declare const _default: ({ view }: Props) => JSX.Element;
export default _default;
export declare const query: import("react-relay").GraphQLTaggedNode;
