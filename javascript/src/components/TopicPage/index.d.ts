import { Component } from 'react';
import { LocationType, NodeTypeOf } from 'components/types';
import { TopicPage_query_Query$data as Response } from '__generated__/TopicPage_query_Query.graphql';
import { TopicPage_topic$data as TopicType } from '__generated__/TopicPage_topic.graphql';
declare type ViewType = Response['view'];
declare type TopicChildType = NodeTypeOf<TopicType['children']>;
declare type Props = {
    alerts: Object[];
    location: LocationType;
    orgLogin: string;
    topic: TopicType;
    view: ViewType;
};
declare type State = {};
declare class TopicPage extends Component<Props, State> {
    constructor(props: Props);
    static getDerivedStateFromProps: (nextProps: Props) => {};
    get children(): (NonNullable<{
        readonly __typename: "Link";
        readonly id: string;
        readonly " $fragmentSpreads": import("relay-runtime").FragmentRefs<"Link_link">;
    } | {
        readonly __typename: "Topic";
        readonly id: string;
        readonly " $fragmentSpreads": import("relay-runtime").FragmentRefs<"Topic_topic">;
    } | {
        readonly __typename: "%other";
    }> | null)[];
    get displaySynonyms(): readonly {
        readonly name: string;
    }[];
    get isGuest(): boolean;
    get recentActivityLocation(): LocationType;
    get linksToReviewLocation(): LocationType;
    renderTopicChild: (child: TopicChildType | null) => JSX.Element | null;
    renderAddForm: () => JSX.Element;
    renderHeadingDetail: () => JSX.Element | null;
    renderNotification: () => JSX.Element;
    renderTopicViews: () => JSX.Element;
    render: () => JSX.Element;
}
export declare const UnwrappedTopicPage: typeof TopicPage;
export declare const query: import("react-relay").GraphQLTaggedNode;
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
