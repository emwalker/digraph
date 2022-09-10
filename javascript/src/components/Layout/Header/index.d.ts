import { Router } from 'found';
import { LocationType } from 'components/types';
declare type Props = {
    location: LocationType;
    router: Router;
    view: any;
    viewer: any;
};
declare const _default: ({ location, router, viewer, view }: Props) => JSX.Element;
export default _default;
