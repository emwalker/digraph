import { RelayProp } from 'react-relay';
import { TopicSearchPage_query_QueryResponse as Response } from '__generated__/TopicSearchPage_query_Query.graphql';
import { TopicSearchPage_topic as TopicType } from '__generated__/TopicSearchPage_topic.graphql';
declare type ViewType = Response['view'];
declare type Props = {
    location: Object;
    orgLogin: string;
    relay: RelayProp;
    router: Object;
    topic: TopicType;
    view: ViewType;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
declare const _default: import("react-relay").Container<Props>;
export default _default;
