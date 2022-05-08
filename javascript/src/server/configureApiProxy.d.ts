import { Express, Request } from 'express';
export interface IGetUserAuthInfoRequest extends Request {
    user: {
        id: string;
        sessionId: string;
    } | undefined;
}
export declare const basicAuthSecret: (viewerId: string, sessionId: string) => string;
declare const _default: (app: Express) => Express;
export default _default;
