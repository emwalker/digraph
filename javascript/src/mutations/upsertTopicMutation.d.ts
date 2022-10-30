import { Dispatch, SetStateAction, KeyboardEvent } from 'react';
import { AlertMessageType } from 'components/types';
import { upsertTopicMutation$data as ResponseType } from '__generated__/upsertTopicMutation.graphql';
export declare type MatchingTopicsType = NonNullable<ResponseType['upsertTopic']>['matchingTopics'];
export declare function makeUpsertTopic({ selectedRepoId, name, setName, topicId, makeAlert }: {
    name: string;
    selectedRepoId: string | null;
    setName: Dispatch<SetStateAction<string>>;
    topicId: string;
    makeAlert: (alert: AlertMessageType, matchingTopics: MatchingTopicsType) => void;
}): (event: KeyboardEvent<HTMLInputElement>) => void;
