import { MatchingTopicsType } from 'mutations/upsertTopicMutation';
import { AlertMessageType } from 'components/types';
declare type Props = {
    alert: AlertMessageType;
    matchingTopics: MatchingTopicsType;
    name: string;
    parentTopicId: string;
    selectedRepoId: string;
};
export default function UpsertTopicAlert({ alert, matchingTopics, selectedRepoId, name, parentTopicId, ...rest }: Props): JSX.Element;
export {};
