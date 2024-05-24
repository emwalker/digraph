import { Location } from 'found';
import { OptionProps } from 'react-select';
export type Edge<T> = {
    node: T | null;
} | null;
export type Color = string;
export type Connection<T> = {
    edges: readonly Edge<T>[] | null;
};
export type EdgesTypeOf<C extends Connection<any>> = C['edges'];
export type EdgeTypeOf<C extends Connection<any>> = NonNullable<EdgesTypeOf<C>>[number];
export type NodeTypeOf<C extends Connection<any>> = NonNullable<EdgeTypeOf<C>>['node'];
export declare function liftEdges<T>(connection: Connection<T>): readonly Edge<T>[];
export declare function liftNodes<T>(connection: Connection<T> | undefined): NonNullable<T>[];
type LocationState = {
    itemTitle: string;
};
export type LocationType = Pick<Location<LocationState>, 'pathname' | 'query' | 'search' | 'state'>;
export interface TopicOption {
    value: string;
    label: string;
    color: string;
}
export interface LinkOption extends OptionProps {
    value: string;
    label: string;
}
export type SynonymType = {
    locale: string;
    name: string;
};
export type AlertType = 'ERROR' | 'WARN' | 'SUCCESS' | '%future added value';
export interface AlertMessageType {
    readonly text: string;
    readonly id: string;
    readonly type: AlertType;
}
export {};
