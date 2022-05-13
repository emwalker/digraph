/// <reference types="react" />
import { Router } from 'found';
import { LocationType } from 'components/types';
import { SearchBox_view as ViewType } from '__generated__/SearchBox_view.graphql';
declare type Props = {
    className?: string;
    location: LocationType;
    router: Router;
    showButton?: boolean;
    view: ViewType;
};
declare const SearchBoxInner: {
    ({ className, router, location, showButton, view }: Props): JSX.Element;
    defaultProps: {
        className: string;
        showButton: boolean;
    };
};
export default SearchBoxInner;
