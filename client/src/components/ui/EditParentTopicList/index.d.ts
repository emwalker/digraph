import React from 'react';
import { TopicOption } from 'components/types';
type SelectedTopics = readonly ({
    label: string;
    value: string;
} | null)[];
export declare const makeOptions: (matches: SelectedTopics) => readonly TopicOption[];
type LoadOptionsType = (str: string) => Promise<readonly TopicOption[]>;
type Props = {
    loadOptions: LoadOptionsType;
    selectedTopics: readonly TopicOption[];
    updateTopics: (topicIds: string[]) => void;
};
export default function EditParentTopicList(props: Props): React.JSX.Element;
export {};
