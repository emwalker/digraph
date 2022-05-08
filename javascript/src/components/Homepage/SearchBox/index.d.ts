import { Component, KeyboardEvent } from 'react';
declare type Props = {
    className?: string | undefined;
    router: {
        push: Function;
    };
};
declare class SearchBox extends Component<Props> {
    static defaultProps: {
        className: string;
    };
    onKeyPress: (event: KeyboardEvent<HTMLInputElement>) => void;
    onSearch: (query: string) => void;
    render: () => JSX.Element;
}
export default SearchBox;
