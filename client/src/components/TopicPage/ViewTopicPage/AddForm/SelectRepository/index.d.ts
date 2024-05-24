import React from 'react';
import { SelectRepository_viewer$key } from '__generated__/SelectRepository_viewer.graphql';
type Props = {
    currentTopicId: string;
    viewer: SelectRepository_viewer$key;
};
export default function SelectRepository({ currentTopicId: parentTopicId, ...rest }: Props): React.JSX.Element;
export {};
