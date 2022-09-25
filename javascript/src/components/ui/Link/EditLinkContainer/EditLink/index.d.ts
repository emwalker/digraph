import { EditLink_link$key } from '__generated__/EditLink_link.graphql';
declare type Props = {
    link: EditLink_link$key;
    refetch: Function;
    toggleForm: () => void;
    viewer: any;
};
export default function EditLink({ refetch, toggleForm, viewer, ...rest }: Props): JSX.Element;
export {};
