import React from 'react';
import { Match } from 'found';
import { TopicPage_query_Query$data } from '__generated__/TopicPage_query_Query.graphql';
export declare const query: import("react-relay").GraphQLTaggedNode;
type ViewType = TopicPage_query_Query$data['view'];
type TopicPageProps = {
    view: ViewType;
    match: Match;
};
export declare function TopicPage({ match: { location }, view }: TopicPageProps): React.JSX.Element;
export {};
