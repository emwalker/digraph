import { Router } from 'found';
import { Homepage_homepage_Query$data as Response } from '__generated__/Homepage_homepage_Query.graphql';
declare type ViewType = Response['view'];
declare type Props = {
    router: Router;
    view: ViewType;
};
declare const Homepage: ({ view, router }: Props) => JSX.Element;
export declare const query: import("react-relay").GraphQLTaggedNode;
export default Homepage;
