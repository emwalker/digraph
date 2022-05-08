import { Component } from 'react';
import { ActionMeta } from 'react-select';
import { TopicOption, Connection } from 'components/types';
export declare const makeOptions: <T>(conn: Connection<T>) => ((T & {
    color: string;
}) | {
    value: string;
    label: string;
    color: string;
})[];
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
