import React from 'react';
import { TopicSearchPage_viewer$key } from '__generated__/TopicSearchPage_viewer.graphql';
import { TopicSearchPage_topic$key } from '__generated__/TopicSearchPage_topic.graphql';
type Props = {
    topic: TopicSearchPage_topic$key;
    viewer: TopicSearchPage_viewer$key;
};
export default function TopicSearchPage(props: Props): React.JSX.Element;
export {};
