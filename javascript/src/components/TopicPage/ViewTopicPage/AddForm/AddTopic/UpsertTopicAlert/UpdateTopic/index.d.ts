import { UpdateTopic_updateTopic$key } from '__generated__/UpdateTopic_updateTopic.graphql';
declare type Props = {
    name: string;
    parentTopicId: string;
    removeAlert: () => void;
    selectedRepoId: string;
    updateTopic: UpdateTopic_updateTopic$key;
};
export default function UpdateTopic({ name, selectedRepoId, parentTopicId, removeAlert, ...rest }: Props): JSX.Element;
export {};
