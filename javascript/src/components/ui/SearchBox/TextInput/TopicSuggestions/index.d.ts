import { ComponentType } from 'react';
import { MentionSuggestionsPubProps } from '@draft-js-plugins/mention/lib/MentionSuggestions/MentionSuggestions';
declare type Props = {
    Suggestions: ComponentType<MentionSuggestionsPubProps>;
    setMentionListOpen: (open: boolean) => void;
    isOpen: boolean;
};
declare const TopicSuggestions: ({ Suggestions, isOpen, setMentionListOpen }: Props) => JSX.Element;
export default TopicSuggestions;
