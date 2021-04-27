import { RelayProp } from 'react-relay';
import { AddForm_topic as Topic } from '__generated__/AddForm_topic.graphql';
import { AddForm_viewer as Viewer } from '__generated__/AddForm_viewer.graphql';
import './index.css';
declare type Props = {
    relay: RelayProp;
    topic: Topic;
    viewer: Viewer;
};
declare const _default: import("react-relay").Container<Props>;
export default _default;
