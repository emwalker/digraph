import React from 'react';
import { SynonymType } from 'components/types';
type Props = {
    synonyms: readonly SynonymType[];
    onDelete: ((position: number) => void) | null;
    onUpdate: (synonyms: SynonymType[]) => void;
};
export default function SortableSynonymList({ synonyms, onUpdate, onDelete }: Props): React.JSX.Element;
export {};
