import { TopicOption } from 'components/types';
declare type SelectedTopics = readonly ({
    label: string;
    value: string;
} | null)[];
export declare const makeOptions: (matches: SelectedTopics) => readonly TopicOption[];
declare type LoadOptionsType = (str: string) => Promise<readonly TopicOption[]>;
declare type Props = {
    loadOptions: LoadOptionsType;
    selectedTopics: readonly TopicOption[];
    updateTopics: (topicIds: string[]) => void;
};
export default function EditParentTopicList(props: Props): JSX.Element;
export {};
