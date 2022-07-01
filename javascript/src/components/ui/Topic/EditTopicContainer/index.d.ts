/// <reference types="react" />
import { RelayProp } from 'react-relay';
declare type Props = {
    isOpen: boolean;
    orgLogin: string;
    relay: RelayProp;
    toggleForm: () => void;
    topicPath: string;
};
declare const EditTopicContainer: ({ isOpen, orgLogin, topicPath, relay, toggleForm }: Props) => JSX.Element;
export default EditTopicContainer;
