import React from 'react';
import { RecentPage_recent_Query$data as Response } from '__generated__/RecentPage_recent_Query.graphql';
type ViewType = Response['view'];
type Props = {
    view: ViewType;
};
declare const _default: ({ view }: Props) => React.JSX.Element;
export default _default;
export declare const query: import("react-relay").GraphQLTaggedNode;
