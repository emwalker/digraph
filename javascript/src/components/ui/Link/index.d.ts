import { Component } from 'react';
import { RelayProp } from 'react-relay';
import { Link_link as LinkType } from '__generated__/Link_link.graphql';
import { Link_view as ViewType } from '__generated__/Link_view.graphql';
import { Link_viewer as ViewerType } from '__generated__/Link_viewer.graphql';
declare type Props = {
    link: LinkType;
    orgLogin: string;
    relay: RelayProp;
    view: ViewType;
    viewer: ViewerType;
};
declare type State = {
    formIsOpen: boolean;
};
declare class Link extends Component<Props, State> {
    constructor(props: Props);
    get repo(): {
        readonly id: string | null;
    };
    get currentRepo(): {
        readonly name: string;
        readonly id: string | null;
    } | null;
    get linkBelongsToCurrentRepo(): boolean;
    get parentTopics(): ({
        readonly displayName: string;
        readonly path: string;
    } | null)[];
    get showEditButton(): boolean;
    toggleForm: () => void;
    render: () => JSX.Element;
}
export declare const UnwrappedLink: typeof Link;
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
