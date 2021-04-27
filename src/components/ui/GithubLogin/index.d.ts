import { Component } from 'react';
declare type Props = {
    className?: string | undefined;
};
declare class GithubLogin extends Component<Props> {
    static defaultProps: {
        className: string;
    };
    onClick: () => void;
    render: () => JSX.Element;
}
export default GithubLogin;
