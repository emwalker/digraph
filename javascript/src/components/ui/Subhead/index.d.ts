declare type Props = {
    heading: string;
    renderHeadingDetail?: Function;
};
declare const Subhead: {
    (props: Props): JSX.Element;
    defaultProps: {
        renderHeadingDetail: null;
    };
};
export default Subhead;
