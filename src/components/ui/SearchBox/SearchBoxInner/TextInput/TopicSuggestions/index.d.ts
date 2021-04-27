import { ComponentType } from 'react';
declare type Props = {
    Suggestions: ComponentType<any>;
    setMentionListOpen: (open: boolean) => void;
};
declare const TopicSuggestions: ({ Suggestions, setMentionListOpen }: Props) => JSX.Element;
export default TopicSuggestions;
