import { SynonymType } from 'components/types';
declare type Props = {
    id: number;
    synonym: SynonymType;
    onDelete?: ((position: number) => void) | null;
};
export default function SortableItem({ id, synonym, onDelete }: Props): JSX.Element;
export {};
