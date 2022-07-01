import { Component, ReactNode } from 'react';
import { Synonym_synonym as SynonymType } from '__generated__/Synonym_synonym.graphql';
declare type Props = {
    dragHandle?: ReactNode;
    onDelete?: (index: number) => void;
    position?: number;
    synonym: SynonymType;
};
declare class Synonym extends Component<Props> {
    static defaultProps: {
        onDelete: undefined;
    };
    onClick: () => void;
    renderDeleteButton: () => JSX.Element;
    render: () => JSX.Element;
}
export declare const UnwrappedSynonym: typeof Synonym;
declare const _default: import("react-relay").Container<Pick<Omit<Props, "relay">, "position" | "synonym" | "dragHandle"> & {
    onDelete?: ((index: number) => void) | undefined;
} & {}>;
export default _default;
