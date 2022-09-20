import { Location } from 'found';
import { TopicPage_query_Query$variables } from '__generated__/TopicPage_query_Query.graphql';
declare type Props = {
    location: Location;
    variables: TopicPage_query_Query$variables;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
export default function TopicPage({ location, variables }: Props): JSX.Element | null;
export {};
