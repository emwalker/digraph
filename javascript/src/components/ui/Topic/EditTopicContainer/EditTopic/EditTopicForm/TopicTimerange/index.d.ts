import { RelayProp } from 'react-relay';
import { TopicTimerange_topicDetail as TopicDetailType } from '__generated__/TopicTimerange_topicDetail.graphql';
declare type Props = {
    topicDetail: TopicDetailType;
    relay: RelayProp;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
