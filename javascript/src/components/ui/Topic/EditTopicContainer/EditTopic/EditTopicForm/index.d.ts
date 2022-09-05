import { RelayRefetchProp } from 'react-relay';
import { EditTopicForm_topicDetail as TopicDetailType } from '__generated__/EditTopicForm_topicDetail.graphql';
declare type Props = {
    isOpen: boolean;
    relay: RelayRefetchProp;
    toggleForm: () => void;
    topicDetail: TopicDetailType;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
