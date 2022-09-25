import { EditRepoTopic_repoTopic$data as RepoTopicType } from '__generated__/EditRepoTopic_repoTopic.graphql';
import { TopicOption } from 'components/types';
declare type SynonymMatches = RepoTopicType['availableTopics']['synonymMatches'];
declare type SelectedTopics = ({
    label: string;
    value: string;
} | null)[];
export declare const makeOptions: (matches: SynonymMatches | SelectedTopics) => TopicOption[];
declare type LoadOptionsType = (str: string) => Promise<readonly TopicOption[]>;
declare type Props = {
    loadOptions: LoadOptionsType;
    selectedTopics: readonly TopicOption[];
    updateTopics: (topicIds: string[]) => void;
};
export default function EditParentTopicList(props: Props): JSX.Element;
export {};
