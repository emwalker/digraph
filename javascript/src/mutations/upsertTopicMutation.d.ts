import { Dispatch, SetStateAction } from 'react';
import { AlertMessageType } from 'components/types';
import { upsertTopicMutation$data as ResponseType, OnMatchingSynonym as OnMatchingSynonymType } from '__generated__/upsertTopicMutation.graphql';
export declare type MatchingTopicsType = NonNullable<ResponseType['upsertTopic']>['matchingTopics'];
declare type Props<E> = {
    ignoreEvent?: (event: E) => boolean;
    makeAlert: (alert: AlertMessageType, matchingTopics: MatchingTopicsType) => void;
    name: string;
    onMatchingSynonym: OnMatchingSynonymType;
    parentTopicId: string;
    selectedRepoId: string | null;
    setName?: Dispatch<SetStateAction<string>>;
    updateTopicId?: string;
};
export declare function makeUpsertTopic<E>({ ignoreEvent, makeAlert, name, onMatchingSynonym, selectedRepoId, setName, parentTopicId, updateTopicId, }: Props<E>): (event: E) => void;
export {};
