import { SynonymType } from 'components/types';
declare type Props = {
    synonyms: readonly SynonymType[];
    onDelete: ((position: number) => void) | null;
    onUpdate: (synonyms: SynonymType[]) => void;
};
export default function SortableSynonymList({ synonyms, onUpdate, onDelete }: Props): JSX.Element;
export {};
