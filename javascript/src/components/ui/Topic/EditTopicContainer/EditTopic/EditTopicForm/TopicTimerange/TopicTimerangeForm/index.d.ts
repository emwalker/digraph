import { RelayProp } from 'react-relay';
import { TopicTimerangeForm_topic as TopicType } from '__generated__/TopicTimerangeForm_topic.graphql';
declare type Props = {
    relay: RelayProp;
    topic: TopicType;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
