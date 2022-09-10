import { Component } from 'react';
import { Router } from 'found';
import { LocationType } from 'components/types';
import { DesktopNav_viewer$data as Viewer } from '__generated__/DesktopNav_viewer.graphql';
declare type Props = {
    className?: string | undefined;
    location: LocationType;
    router: Router;
    view: any;
    viewer: Viewer;
};
declare class DesktopNav extends Component<Props> {
    static defaultProps: {
        className: string;
    };
    get className(): string;
    get isGuest(): boolean;
    render: () => JSX.Element;
}
export declare const UnwrappedDesktopNav: typeof DesktopNav;
declare const _default: import("react-relay").Container<Pick<Omit<Props, "relay">, "view" | "router" | "location" | "viewer"> & {
    className?: string | undefined;
} & {}>;
export default _default;
