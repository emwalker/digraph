import { EditRepoLink_repoLink$key } from '__generated__/EditRepoLink_repoLink.graphql';
import { EditRepoLink_viewer$key } from '__generated__/EditRepoLink_viewer.graphql';
declare type Props = {
    refetch: Function;
    repoLink: EditRepoLink_repoLink$key;
    viewer: EditRepoLink_viewer$key;
};
export default function EditRepoLink({ refetch, ...rest }: Props): JSX.Element;
export {};
