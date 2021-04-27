import { Express } from 'express';
declare const registerEndpointsFn: (provider: string) => (app: Express) => Express;
export default registerEndpointsFn;
