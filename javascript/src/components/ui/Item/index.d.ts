import { ReactNode } from 'react';
import { Color } from 'components/types';
declare type Topic = {
    displayName: string;
    id: string;
};
declare type Props = {
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
export default function Item(props: Props): JSX.Element;
export {};
