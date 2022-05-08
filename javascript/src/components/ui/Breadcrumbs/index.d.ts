import type { Breadcrumbs_repository as Repository } from '__generated__/Breadcrumbs_repository.graphql';
declare type Props = {
    orgLogin: string;
    repository: Repository | null;
};
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
