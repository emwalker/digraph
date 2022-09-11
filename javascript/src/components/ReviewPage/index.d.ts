import { ReviewPage_view$key } from '__generated__/ReviewPage_view.graphql';
import { ViewType as QueryViewType } from './reviewPageQuery';
declare type Props = {
    view: ReviewPage_view$key;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
export declare type ContainerViewType = QueryViewType;
declare const _default: ({ view }: Props) => JSX.Element;
export default _default;
