import React from 'react';
import { Match } from 'found';
import { UserSettings_view$key } from '__generated__/UserSettings_view.graphql';
export declare const query: import("react-relay").GraphQLTaggedNode;
type Props = {
    match: Match;
    view: UserSettings_view$key;
};
declare const _default: ({ view, match }: Props) => React.JSX.Element | null;
export default _default;
