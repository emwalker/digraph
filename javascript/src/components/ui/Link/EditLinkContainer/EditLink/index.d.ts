/// <reference types="react" />
declare type CallerProps = {
    isOpen: boolean;
    toggleForm: () => void;
};
declare type RenderArgs = {
    error: Error | null;
    props?: unknown;
};
declare const _default: ({ isOpen, toggleForm }: CallerProps) => ({ error, props: renderProps }: RenderArgs) => JSX.Element | null;
export default _default;
