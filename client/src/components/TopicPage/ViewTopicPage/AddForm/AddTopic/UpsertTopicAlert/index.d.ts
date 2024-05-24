import React from 'react';
import { MatchingTopicsType } from 'mutations/upsertTopicMutation';
import { AlertMessageType } from 'components/types';
type Props = {
    alert: AlertMessageType;
    matchingTopics: MatchingTopicsType;
    name: string;
    parentTopicId: string;
    selectedRepoId: string;
};
export default function UpsertTopicAlert({ alert, matchingTopics, selectedRepoId, name, parentTopicId, }: Props): React.JSX.Element;
export {};
