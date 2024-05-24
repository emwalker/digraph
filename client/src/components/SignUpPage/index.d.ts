import React, { Component } from 'react';
type State = {
    gCaptchaResponse: string | null;
};
type Props = {};
declare class SignUpPage extends Component<Props, State> {
    constructor(props: Props);
    onCaptchaSumit: (token: string | null) => void;
    renderCreateAccountButton: () => React.JSX.Element;
    render: () => React.JSX.Element;
}
export default SignUpPage;
