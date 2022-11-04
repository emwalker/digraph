declare type Props = {
    name: string;
    parentTopicId: string;
    selectedRepoId: string;
};
export default function CreateTopic({ selectedRepoId, name, parentTopicId }: Props): JSX.Element;
export {};
