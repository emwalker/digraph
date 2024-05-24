import React from 'react';
import { SynonymType } from 'components/types';
type Props = {
    canUpdate: boolean;
    onDelete: (position: number) => void;
    onUpdate: (synonyms: SynonymType[]) => void;
    synonyms: readonly SynonymType[];
};
export default function SynonymList({ canUpdate, onDelete, onUpdate, synonyms }: Props): React.JSX.Element;
export {};
