import { Dispatch, SetStateAction, KeyboardEvent } from 'react';
import { AlertMessageType } from 'components/types';
export declare function makeUpsertTopic({ selectedRepoId, name, setName, topicId, makeAlert }: {
    name: string;
    selectedRepoId: string | null;
    setName: Dispatch<SetStateAction<string>>;
    topicId: string;
    makeAlert: (alert: AlertMessageType) => void;
}): (event: KeyboardEvent<HTMLInputElement>) => void;
