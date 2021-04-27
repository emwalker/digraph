import { RelayProp } from 'react-relay';
import { AddLink_viewer as ViewerType } from '__generated__/AddLink_viewer.graphql';
import { AddLink_topic as TopicType } from '__generated__/AddLink_topic.graphql';
declare type Props = {
    disabled?: boolean;
    relay: RelayProp;
    topic: TopicType;
    viewer: ViewerType;
};
declare const _default: import("react-relay").Container<Props>;
export default _default;
