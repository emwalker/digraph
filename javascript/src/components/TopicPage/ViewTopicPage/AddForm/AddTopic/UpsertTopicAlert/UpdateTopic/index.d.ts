import { UpdateTopic_updateTopic$key } from '__generated__/UpdateTopic_updateTopic.graphql';
declare type Props = {
    updateTopic: UpdateTopic_updateTopic$key;
    parentTopicId: string;
    selectedRepoId: string;
    name: string;
};
export default function UpdateTopic({ name, selectedRepoId, parentTopicId, ...rest }: Props): JSX.Element;
export {};
