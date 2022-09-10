import { RelayProp } from 'react-relay';
import { AddLink_viewer$data as ViewerType } from '__generated__/AddLink_viewer.graphql';
import { AddLink_topic$data as TopicType } from '__generated__/AddLink_topic.graphql';
declare type Props = {
    disabled?: boolean;
    relay: RelayProp;
    topic: TopicType;
    viewer: ViewerType;
};
declare const _default: import("react-relay").Container<Pick<Omit<Props, "relay">, "topic" | "viewer"> & {
    disabled?: boolean | undefined;
} & {}>;
export default _default;
