import { RelayRefetchProp } from 'react-relay';
import { EditTopicForm_topic$data as TopicType } from '__generated__/EditTopicForm_topic.graphql';
import { EditTopicForm_viewer$data as ViewerType } from '__generated__/EditTopicForm_viewer.graphql';
declare type Props = {
    isOpen: boolean;
    relay: RelayRefetchProp;
    toggleForm: () => void;
    topic: TopicType;
    viewer: ViewerType;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
