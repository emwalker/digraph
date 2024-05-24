import React from 'react';
import { AddTopic_viewer$key } from '__generated__/AddTopic_viewer.graphql';
import { AddTopic_parentTopic$key } from '__generated__/AddTopic_parentTopic.graphql';
type Props = {
    disabled?: boolean;
    parentTopic: AddTopic_parentTopic$key;
    viewer: AddTopic_viewer$key;
};
export default function AddTopic(props: Props): React.JSX.Element | null;
export {};
