import { Router } from 'found';
import { LocationType } from 'components/types';
import { DesktopNav_viewer$key } from '__generated__/DesktopNav_viewer.graphql';
declare type Props = {
    location: LocationType;
    router: Router;
    view: any;
    viewer: DesktopNav_viewer$key;
};
export default function DesktopNav(props: Props): JSX.Element;
export {};
