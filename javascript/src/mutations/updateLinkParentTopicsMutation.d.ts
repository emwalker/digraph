export declare function makeUpdateLinkParentTopicsCallback({ linkId, selectedRepoId }: {
    linkId: string;
    selectedRepoId: string | null;
}): (parentTopicIds: string[]) => void;
