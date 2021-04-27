import { RelayRefetchProp } from 'react-relay';
import { EditTopicForm_topic as TopicType } from '__generated__/EditTopicForm_topic.graphql';
declare type Props = {
    isOpen: boolean;
    orgLogin: string;
    relay: RelayRefetchProp;
    toggleForm: () => void;
    topic: TopicType;
};
declare const _default: import("react-relay").Container<Props>;
export default _default;
