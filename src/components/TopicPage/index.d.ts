import { Component } from 'react';
import { RelayProp } from 'react-relay';
import { LocationType, NodeTypeOf } from 'components/types';
import { TopicPage_query_QueryResponse as Response } from '__generated__/TopicPage_query_Query.graphql';
import { TopicPage_topic as TopicType } from '__generated__/TopicPage_topic.graphql';
declare type ViewType = Response['view'];
declare type LinkType = NodeTypeOf<TopicType['links']>;
declare type ChildTopicType = NodeTypeOf<TopicType['childTopics']>;
declare type Props = {
    alerts: Object[];
    location: LocationType;
    orgLogin: string;
    relay: RelayProp;
    router: Object;
    topic: TopicType;
    view: ViewType;
};
declare type State = {};
declare class TopicPage extends Component<Props, State> {
    constructor(props: Props);
    static getDerivedStateFromProps: (nextProps: Props) => {};
    get links(): ({
        readonly id: string;
        readonly " $fragmentRefs": import("relay-runtime").FragmentRefs<"Link_link">;
    } | null)[];
    get topics(): ({
        readonly id: string;
        readonly " $fragmentRefs": import("relay-runtime").FragmentRefs<"Topic_topic">;
    } | null)[];
    get synonyms(): readonly {
        readonly name: string;
    }[];
    get isGuest(): boolean;
    get repoName(): string;
    get recentActivityLocation(): LocationType;
    get linksToReviewLocation(): LocationType;
    renderLink: (link: LinkType | null) => JSX.Element | null;
    renderTopic: (topic: ChildTopicType | null) => JSX.Element | null;
    renderAddForm: () => JSX.Element;
    renderHeadingDetail: () => JSX.Element | null;
    renderNotification: () => JSX.Element;
    renderTopicViews: () => JSX.Element;
    render: () => JSX.Element;
}
export declare const UnwrappedTopicPage: typeof TopicPage;
export declare const query: import("react-relay").GraphQLTaggedNode;
declare const _default: import("react-relay").Container<Props>;
export default _default;
