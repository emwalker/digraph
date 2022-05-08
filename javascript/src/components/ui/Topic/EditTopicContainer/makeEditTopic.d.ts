declare type Props = {
    isOpen: boolean;
    orgLogin: string;
    toggleForm: () => void;
};
declare type RenderProps = {
    error: Error;
    props: any;
};
declare const _default: ({ isOpen, orgLogin, toggleForm }: Props) => ({ error, props }: RenderProps) => JSX.Element;
export default _default;
