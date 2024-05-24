import React from 'react';
import { Router } from 'found';
import { LocationType } from 'components/types';
type Props = {
    location: LocationType;
    router: Router;
    view: any;
    viewer: any;
};
declare const _default: ({ location, router, viewer, view }: Props) => React.JSX.Element;
export default _default;
