import { Location } from 'found';
import { ViewTopicPage_viewer$key } from '__generated__/ViewTopicPage_viewer.graphql';
import { ViewTopicPage_topic$key } from '__generated__/ViewTopicPage_topic.graphql';
declare type Props = {
    location: Location;
    topic: ViewTopicPage_topic$key;
    viewer: ViewTopicPage_viewer$key;
};
export default function TopicPage(props: Props): JSX.Element;
export {};
