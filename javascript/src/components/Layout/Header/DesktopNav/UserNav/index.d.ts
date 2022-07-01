/// <reference types="react" />
import { UserNav_viewer as Viewer } from '__generated__/UserNav_viewer.graphql';
declare type Props = {
    viewer: Viewer;
};
export declare const UnwrappedUserNav: ({ viewer }: Props) => JSX.Element;
declare const _default: import("react-relay").Container<Omit<Props, "relay">>;
export default _default;
