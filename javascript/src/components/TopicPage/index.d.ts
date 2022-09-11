import { LocationType } from 'components/types';
import { TopicPage_query_Query$data as Response } from '__generated__/TopicPage_query_Query.graphql';
import { TopicPage_topic$key } from '__generated__/TopicPage_topic.graphql';
declare type ViewType = Response['view'];
declare type Props = {
    alerts: Object[];
    location: LocationType;
    orgLogin: string;
    topic: TopicPage_topic$key;
    view: ViewType;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
export default function TopicPage(props: Props): JSX.Element;
export {};
