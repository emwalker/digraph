import { Router } from 'found';
import { LocationType } from 'components/types';
import { SearchBox_view$key } from '__generated__/SearchBox_view.graphql';
declare type Props = {
    className?: string;
    location: LocationType;
    router: Router;
    showButton?: boolean;
    view: SearchBox_view$key;
};
export default function SearchBox({ router, className, showButton, location, ...rest }: Props): JSX.Element;
export {};
