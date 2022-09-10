import { Component } from 'react';
import { Router } from 'found';
import { LocationType } from 'components/types';
import { MobileNav_viewer$data as Viewer } from '__generated__/MobileNav_viewer.graphql';
declare type Props = {
    location: LocationType;
    router: Router;
    showButton?: boolean;
    viewer: Viewer;
};
declare type State = {
    isOpen: boolean;
};
declare class MobileNav extends Component<Props, State> {
    static defaultProps: {
        showButton: boolean;
    };
    constructor(props: Props);
    onClick: () => void;
    render: () => JSX.Element;
}
export declare const UnwrappedMobileNav: typeof MobileNav;
declare const _default: import("react-relay").Container<Pick<Omit<Props, "relay">, "router" | "location" | "viewer"> & {
    showButton?: boolean | undefined;
} & {}>;
export default _default;
