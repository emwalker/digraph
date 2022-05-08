declare type CallerProps = {
    isOpen: boolean;
    orgLogin: string;
    toggleForm: () => void;
};
declare type RenderArgs = {
    error: Error | null;
    props?: unknown;
};
declare const _default: ({ isOpen, orgLogin, toggleForm }: CallerProps) => ({ error, props: renderProps }: RenderArgs) => JSX.Element | null;
export default _default;
