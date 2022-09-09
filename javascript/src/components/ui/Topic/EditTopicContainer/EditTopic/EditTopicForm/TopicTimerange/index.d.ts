import { RelayProp } from 'react-relay';
import { TopicTimerange_repoTopic as RepoTopicType } from '__generated__/TopicTimerange_repoTopic.graphql';
declare type Props = {
    topicDetail: RepoTopicType;
    relay: RelayProp;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
