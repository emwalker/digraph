import React from 'react';
import { SynonymType } from 'components/types';
type Props = {
    id: number;
    synonym: SynonymType;
    onDelete?: ((position: number) => void) | null;
};
export default function SortableItem({ id, synonym, onDelete }: Props): React.JSX.Element;
export {};
