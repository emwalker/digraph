import { RelayProp } from 'react-relay';
import { TopicTimerange_topic as TopicType } from '__generated__/TopicTimerange_topic.graphql';
declare type Props = {
    topic: TopicType;
    relay: RelayProp;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
