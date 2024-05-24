import React, { Component } from 'react';
type Props = {
    className?: string | undefined;
};
declare class GithubLogin extends Component<Props> {
    static defaultProps: {
        className: string;
    };
    onClick: () => void;
    render: () => React.JSX.Element;
}
export default GithubLogin;
