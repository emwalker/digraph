declare type Props = {
    linkId: string;
    selectedRepoId: string;
};
export declare function makeUpdateLinkParentTopicsCallback({ linkId, selectedRepoId }: Props): (parentTopicIds: string[]) => void;
export {};
