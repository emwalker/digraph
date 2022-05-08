import { Component } from 'react';
declare type State = {
    gCaptchaResponse: string | null;
};
declare type Props = {};
declare class SignUpPage extends Component<Props, State> {
    constructor(props: Props);
    onCaptchaSumit: (token: string | null) => void;
    renderCreateAccountButton: () => JSX.Element;
    render: () => JSX.Element;
}
export default SignUpPage;
