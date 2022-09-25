import { PreloadedQuery } from 'react-relay';
import { EditLinkQuery as EditLinkQueryType } from '__generated__/EditLinkQuery.graphql';
declare type Props = {
    queryRef: PreloadedQuery<EditLinkQueryType>;
    toggleForm: () => void;
};
export default function EditLinkContainer(props: Props): JSX.Element | null;
export {};
