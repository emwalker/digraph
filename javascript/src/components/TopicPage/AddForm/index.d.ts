import { AddForm_topic$data as Topic } from '__generated__/AddForm_topic.graphql';
import { AddForm_viewer$data as Viewer } from '__generated__/AddForm_viewer.graphql';
import './index.css';
declare type Props = {
    topic: Topic;
    viewer: Viewer;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
