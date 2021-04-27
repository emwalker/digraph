import { ReactNode } from 'react';
import { Match, Router } from 'found';
import { LayoutQueryResponse } from '__generated__/LayoutQuery.graphql';
declare type AlertsType = LayoutQueryResponse['alerts'];
declare type ViewType = LayoutQueryResponse['view'];
declare type Props = {
    alerts: AlertsType;
    children?: ReactNode;
    router: Router;
    match: Match;
    view: ViewType;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
declare const Layout: ({ alerts, children, view, match, router }: Props) => JSX.Element;
export default Layout;
