import { TopicSearchPage_query_Query$data as Response } from '__generated__/TopicSearchPage_query_Query.graphql';
import { TopicSearchPage_topic$key } from '__generated__/TopicSearchPage_topic.graphql';
declare type ViewType = Response['view'];
declare type Props = {
    orgLogin: string;
    topic: TopicSearchPage_topic$key;
    view: ViewType;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
export default function TopicSearchPage(props: Props): JSX.Element;
export {};
