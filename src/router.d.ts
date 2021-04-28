import { createRender } from 'found';
import { Store, Action, Middleware, Dispatch, AnyAction } from 'redux';
import { FetcherBase } from './FetcherBase';
declare type RouteStore = Store<any, Action<any>>;
export declare const historyMiddlewares: Middleware<{}, any, Dispatch<AnyAction>>[];
export declare function createResolver(fetcher: FetcherBase): any;
export declare const createRouteConfig: (store: RouteStore) => import("found").RouteConfig;
declare type RenderType = ReturnType<typeof createRender>;
export declare const render: RenderType;
export {};
