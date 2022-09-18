import { SynonymType, SortableType } from 'components/types';
declare type Props = {
    synonyms: Record<string, SortableType<SynonymType>>;
    onDelete: ((position: number) => void) | null;
    onUpdate: (synonyms: SynonymType[]) => void;
};
export default function SortableSynonymList(props: Props): JSX.Element;
export {};
