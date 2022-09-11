import { Router } from 'found';
import { LocationType } from 'components/types';
import { MobileNav_viewer$key } from '__generated__/MobileNav_viewer.graphql';
declare type Props = {
    location: LocationType;
    router: Router;
    showButton?: boolean;
    viewer: MobileNav_viewer$key;
};
export default function MobileNav(props: Props): JSX.Element;
export {};
