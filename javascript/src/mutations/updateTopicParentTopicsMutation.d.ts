declare type Props = {
    repoId: string | null;
    topicId: string;
};
export declare function makeUpdateTopicParentTopicsCallback({ repoId, topicId }: Props): (parentTopicIds: string[]) => void;
export {};
