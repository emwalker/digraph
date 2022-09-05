import { Component } from 'react';
import { ActionMeta } from 'react-select';
import { EditTopicForm_topicDetail as TopicDetailType } from '__generated__/EditTopicForm_topicDetail.graphql';
import { TopicOption } from 'components/types';
declare type SynonymMatches = TopicDetailType['availableTopics']['synonymMatches'];
declare type SelectedTopics = ({
    label: string;
    value: string;
} | null)[];
export declare const makeOptions: (matches: SynonymMatches | SelectedTopics) => TopicOption[];
declare type LoadOptionsType = (str: string) => Promise<readonly TopicOption[]>;
declare type Props = {
    loadOptions: LoadOptionsType;
    selectedTopics: readonly TopicOption[];
    updateTopics: (topics: string[]) => void;
};
declare type State = {
    inputValue: string;
    selectedTopics: readonly TopicOption[];
};
declare class EditTopicList extends Component<Props, State> {
    loadOptions: LoadOptionsType;
    constructor(props: Props);
    onInputChange: (newValue: string) => void;
    onChange: (selectedTopics: readonly TopicOption[], action: ActionMeta<TopicOption>) => void;
    render: () => JSX.Element;
}
export default EditTopicList;
