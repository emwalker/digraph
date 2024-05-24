import React, { ComponentType } from 'react';
import { MentionSuggestionsPubProps } from '@draft-js-plugins/mention/lib/MentionSuggestions/MentionSuggestions';
type Props = {
    Suggestions: ComponentType<MentionSuggestionsPubProps>;
    setMentionListOpen: (open: boolean) => void;
    isOpen: boolean;
};
declare const TopicSuggestions: ({ Suggestions, isOpen, setMentionListOpen }: Props) => React.JSX.Element;
export default TopicSuggestions;
