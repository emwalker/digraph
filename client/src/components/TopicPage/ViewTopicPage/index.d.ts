import React from 'react';
import { Location } from 'found';
import { ViewTopicPage_viewer$key } from '__generated__/ViewTopicPage_viewer.graphql';
import { ViewTopicPage_topic$key } from '__generated__/ViewTopicPage_topic.graphql';
type Props = {
    location: Location;
    topic: ViewTopicPage_topic$key;
    viewer: ViewTopicPage_viewer$key;
};
export default function ViewTopicPage(props: Props): React.JSX.Element;
export {};
