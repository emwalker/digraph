import { RelayProp } from 'react-relay';
import { Topic_topic as TopicType } from '__generated__/Topic_topic.graphql';
import { Topic_view as ViewType } from '__generated__/Topic_view.graphql';
declare type Props = {
    orgLogin: string;
    relay: RelayProp;
    topic: TopicType;
    view: ViewType;
};
declare const _default: import("react-relay").Container<Props>;
export default _default;
