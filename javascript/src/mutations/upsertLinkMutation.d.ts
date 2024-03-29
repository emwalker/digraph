import { Dispatch, SetStateAction } from 'react';
declare type Props = {
    linkId?: string | null;
    selectedRepoId: string | null;
    setUrl?: Dispatch<SetStateAction<string>>;
    title?: string | null;
    topicId?: string | null;
    url: string | null;
};
export declare function makeUpsertLinkCallback({ linkId, selectedRepoId, setUrl, title, topicId, url, }: Props): () => void;
export {};
