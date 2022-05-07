import { TopicSearchPage_query_QueryResponse as Response } from '__generated__/TopicSearchPage_query_Query.graphql';
import { TopicSearchPage_topic as TopicType } from '__generated__/TopicSearchPage_topic.graphql';
declare type ViewType = Response['view'];
declare type Props = {
    orgLogin: string;
    topic: TopicType;
    view: ViewType;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
