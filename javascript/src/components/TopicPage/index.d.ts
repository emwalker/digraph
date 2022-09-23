import { Location } from 'found';
import { TopicPage_query_Query$data } from '__generated__/TopicPage_query_Query.graphql';
export declare const query: import("react-relay").GraphQLTaggedNode;
declare type TopicPageProps = {
    data: TopicPage_query_Query$data;
    location: Location;
};
export declare function TopicPage(props: TopicPageProps): JSX.Element | null;
export {};
