declare type Props = {
    isOpen: boolean;
    toggleForm: () => void;
    topicId: string;
    viewerId: string | null;
};
export default function EditTopicContainer({ isOpen, topicId, toggleForm, viewerId }: Props): JSX.Element | null;
export {};
