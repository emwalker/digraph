import { SelectedRepo_viewer$data as Viewer } from '__generated__/SelectedRepo_viewer.graphql';
declare type Props = {
    viewer: Viewer;
};
export declare const UnwrappedSelectedRepo: ({ viewer }: Props) => JSX.Element | null;
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
