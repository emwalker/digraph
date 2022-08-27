/// <reference types="react" />
import { RelayProp } from 'react-relay';
declare type Props = {
    isOpen: boolean;
    relay: RelayProp;
    toggleForm: () => void;
    topicId: string;
};
declare const EditTopicContainer: ({ isOpen, topicId, relay, toggleForm }: Props) => JSX.Element;
export default EditTopicContainer;
