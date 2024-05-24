import React, { ReactNode } from 'react';
import { Color } from 'components/types';
type Topic = {
    displayName: string;
    id: string;
};
type Props = {
    canEdit: boolean;
    children: ReactNode;
    className: string;
    description?: string | null;
    formIsOpen: boolean;
    newlyAdded: boolean;
    repoColors: Color[];
    showEditButton: boolean;
    showLink?: boolean;
    showRepoOwnership: boolean;
    title: string;
    toggleForm: () => void;
    topics: Topic[];
    url: string | null;
};
export default function Item(props: Props): React.JSX.Element;
export {};
