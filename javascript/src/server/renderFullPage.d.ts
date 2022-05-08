import { ReactNode } from 'react';
import { FetcherBase } from '../FetcherBase';
declare type Assets = {
    '': {
        css: string[];
        js: string[];
    } | undefined;
    client: {
        [key: string]: string;
    };
};
declare const _default: (assets: Assets, fetcher: FetcherBase, element: ReactNode, preloadedState: Object) => Promise<string>;
export default _default;
