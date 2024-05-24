import React from 'react';
import { AddLink_viewer$key } from '__generated__/AddLink_viewer.graphql';
import { AddLink_parentTopic$key } from '__generated__/AddLink_parentTopic.graphql';
type Props = {
    disabled?: boolean;
    parentTopic: AddLink_parentTopic$key;
    viewer: AddLink_viewer$key;
};
export default function AddLink(props: Props): React.JSX.Element;
export {};
