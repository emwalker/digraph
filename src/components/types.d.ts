import { Location } from 'found';
import { OptionTypeBase } from 'react-select';
export declare type Edge<T> = {
    node: T | null;
} | null;
export declare type Connection<T> = {
    edges: readonly Edge<T>[] | null;
};
export declare type EdgesTypeOf<C extends Connection<any>> = C['edges'];
export declare type EdgeTypeOf<C extends Connection<any>> = NonNullable<EdgesTypeOf<C>>[number];
export declare type NodeTypeOf<C extends Connection<any>> = NonNullable<EdgeTypeOf<C>>['node'];
export declare function liftEdges<T>(connection: Connection<T>): readonly Edge<T>[];
export declare function liftNodes<T>(connection: Connection<T> | undefined): (T | null)[];
declare type LocationState = {
    orgLogin: string;
    repoName: string | null;
    itemTitle: string;
};
export declare type LocationType = Pick<Location<LocationState>, 'pathname' | 'query' | 'search' | 'state'>;
export interface TopicOption extends OptionTypeBase {
    value: string;
    label: string;
    color: string;
}
export interface LinkOption extends OptionTypeBase {
    value: string;
    label: string;
}
export declare type SynonymType = {
    name: string;
    locale: string;
};
export declare type AlertType = 'ERROR' | 'WARN' | 'SUCCESS' | '%future added value';
export {};
