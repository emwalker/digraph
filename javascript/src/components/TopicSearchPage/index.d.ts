import { TopicSearchPage_viewer$key } from '__generated__/TopicSearchPage_viewer.graphql';
import { TopicSearchPage_topic$key } from '__generated__/TopicSearchPage_topic.graphql';
declare type Props = {
    orgLogin: string;
    topic: TopicSearchPage_topic$key;
    viewer: TopicSearchPage_viewer$key;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
export default function TopicSearchPage(props: Props): JSX.Element;
export {};
