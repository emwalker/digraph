import { SynonymType } from 'components/types';
declare type Props = {
    canUpdate: boolean;
    onDelete: (position: number) => void;
    onUpdate: (synonyms: SynonymType[]) => void;
    synonyms: readonly SynonymType[];
};
export default function SynonymList(props: Props): JSX.Element;
export {};
