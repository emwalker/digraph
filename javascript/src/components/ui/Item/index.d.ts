import { Component, ReactNode } from 'react';
import { LocationType } from 'components/types';
declare type Topic = {
    displayName: string;
    id: string;
} | null;
declare type Props = {
    canEdit: boolean;
    children: ReactNode;
    className: string;
    description?: string | null;
    displayColor: string | null;
    formIsOpen: boolean;
    newlyAdded: boolean;
    showEditButton: boolean | null;
    showLink?: boolean;
    title: string;
    toggleForm: () => void;
    topics: Topic[];
    url: string | null;
};
declare class Item extends Component<Props> {
    static defaultProps: {
        description: null;
        showLink: boolean;
    };
    get className(): string;
    get showEditButton(): boolean;
    get style(): {
        borderLeftColor?: undefined;
    } | {
        borderLeftColor: string;
    };
    get url(): JSX.Element | null;
    get titleLink(): JSX.Element;
    locationDescriptor: (pathname: string, itemTitle: string) => LocationType;
    renderTopicBadge: (topic: Topic) => JSX.Element | null;
    renderEditable: () => JSX.Element;
    renderWide: () => JSX.Element;
    render: () => JSX.Element;
}
export default Item;
