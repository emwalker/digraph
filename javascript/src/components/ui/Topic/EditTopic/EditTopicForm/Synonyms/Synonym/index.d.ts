import { ReactNode } from 'react';
import { Synonym_synonym$key as SynonymType } from '__generated__/Synonym_synonym.graphql';
declare type Props = {
    dragHandle?: ReactNode;
    onDelete?: (index: number) => void;
    position?: number;
    synonym: SynonymType;
};
export default function Synonym(props: Props): JSX.Element;
export {};
