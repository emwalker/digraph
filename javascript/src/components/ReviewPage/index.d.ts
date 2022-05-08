import { ViewType as QueryViewType } from './reviewPageQuery';
export declare const query: import("react-relay").GraphQLTaggedNode;
export declare type ContainerViewType = QueryViewType;
declare type RenderProps = {
    view: QueryViewType;
};
declare const _default: ({ view }: RenderProps) => JSX.Element;
export default _default;
