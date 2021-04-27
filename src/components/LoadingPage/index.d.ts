import { Router } from 'found';
import { LocationType } from 'components/types';
declare type Props = {
    location: LocationType;
    router: Router;
};
declare const LoadingPage: ({ location, router }: Props) => JSX.Element;
export default LoadingPage;
