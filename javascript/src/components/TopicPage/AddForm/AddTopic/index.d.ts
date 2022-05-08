import { RelayProp } from 'react-relay';
import { AddTopic_viewer as Viewer } from '__generated__/AddTopic_viewer.graphql';
import { AddTopic_topic as Topic } from '__generated__/AddTopic_topic.graphql';
declare type Props = {
    disabled?: boolean;
    relay: RelayProp;
    topic: Topic;
    viewer: Viewer;
};
declare const _default: import("react-relay").Container<Pick<Omit<Props, "relay">, "topic" | "viewer"> & {
    disabled?: boolean | undefined;
} & {}>;
export default _default;
