import { Component } from 'react';
import { RelayProp } from 'react-relay';
import { Link_link$data as LinkType } from '__generated__/Link_link.graphql';
import { Link_viewer$data as ViewerType } from '__generated__/Link_viewer.graphql';
declare type Props = {
    link: LinkType;
    relay: RelayProp;
    viewer: ViewerType;
};
declare type State = {
    formIsOpen: boolean;
};
declare class Link extends Component<Props, State> {
    constructor(props: Props);
    get linkBelongsToCurrentRepo(): boolean;
    get parentTopics(): ({
        readonly displayName: string;
        readonly id: string;
    } | null)[];
    get showEditButton(): boolean;
    toggleForm: () => void;
    render: () => JSX.Element;
}
export declare const UnwrappedLink: typeof Link;
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
