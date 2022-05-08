import { Router } from 'found';
import { LocationType } from 'components/types';
import { LayoutQueryResponse as Response } from '__generated__/LayoutQuery.graphql';
declare type ViewType = Response['view'];
declare type ViewerType = ViewType['viewer'];
declare type Props = {
    location: LocationType;
    router: Router;
    view: ViewType;
    viewer: ViewerType;
};
declare const _default: ({ location, router, viewer, view }: Props) => JSX.Element;
export default _default;
