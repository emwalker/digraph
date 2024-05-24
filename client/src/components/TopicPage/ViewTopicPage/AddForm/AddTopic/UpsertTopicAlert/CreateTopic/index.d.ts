import React from 'react';
type Props = {
    name: string;
    parentTopicId: string;
    removeAlert: () => void;
    selectedRepoId: string;
};
export default function CreateTopic({ selectedRepoId, name, parentTopicId, removeAlert }: Props): React.JSX.Element;
export {};
