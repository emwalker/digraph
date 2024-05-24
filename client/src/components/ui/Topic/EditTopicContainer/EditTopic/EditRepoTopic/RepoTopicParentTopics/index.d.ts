import React from 'react';
import { RepoTopicParentTopics_repoTopic$key } from '__generated__/RepoTopicParentTopics_repoTopic.graphql';
type Props = {
    repoTopic: RepoTopicParentTopics_repoTopic$key;
    selectedRepoId: string;
    viewerId: string;
};
export default function RepoTopicParentTopics({ selectedRepoId, viewerId, ...rest }: Props): React.JSX.Element;
export {};
