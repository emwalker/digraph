import { Dispatch, SetStateAction, KeyboardEvent } from 'react';
export declare function makeUpsertTopic({ selectedRepoId, name, setName, topicId }: {
    name: string;
    selectedRepoId: string | null;
    setName: Dispatch<SetStateAction<string>>;
    topicId: string;
}): (event: KeyboardEvent<HTMLInputElement>) => void;
