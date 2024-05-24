import React, { ReactNode } from 'react';
import { Match, Router } from 'found';
import { LayoutQuery$data as Response } from '__generated__/LayoutQuery.graphql';
type AlertsType = Response['alerts'];
type ViewType = Response['view'];
type Props = {
    alerts: AlertsType;
    children?: ReactNode;
    router: Router;
    match: Match;
    view: ViewType;
};
export declare const query: import("react-relay").GraphQLTaggedNode;
export default function Layout({ alerts, children, view, match, router }: Props): React.JSX.Element;
export {};
