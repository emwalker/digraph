declare type Props = {
    isOpen: boolean;
    toggleForm: () => void;
    topicId: string;
};
export default function EditTopicContainer({ isOpen, topicId, toggleForm }: Props): JSX.Element | null;
export {};
