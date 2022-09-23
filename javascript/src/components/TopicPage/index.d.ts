import { Match } from 'found';
import { TopicPage_query_Query$data } from '__generated__/TopicPage_query_Query.graphql';
export declare const query: import("react-relay").GraphQLTaggedNode;
declare type ViewType = TopicPage_query_Query$data['view'];
declare type TopicPageProps = {
    view: ViewType;
    match: Match;
};
export declare function TopicPage({ match: { location }, view }: TopicPageProps): JSX.Element;
export {};
