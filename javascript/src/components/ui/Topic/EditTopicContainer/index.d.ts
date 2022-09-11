import { PreloadedQuery } from 'react-relay';
import { EditTopicQuery as EditTopicQueryType } from '__generated__/EditTopicQuery.graphql';
declare type Props = {
    queryRef: PreloadedQuery<EditTopicQueryType>;
    toggleForm: () => void;
};
export default function EditTopicContainer(props: Props): JSX.Element | null;
export {};
