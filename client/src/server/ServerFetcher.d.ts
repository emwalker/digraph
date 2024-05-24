import { RequestParameters, Variables } from 'relay-runtime';
import { FetcherBase } from '../FetcherBase';
type Headers = {
    Authorization?: string;
    'Content-Type': string;
};
declare class ServerFetcher extends FetcherBase {
    constructor();
    payloads: {
        [key: string]: string;
    };
    sessionId: string | undefined | null;
    viewerId: string | undefined | null;
    setBasicAuth(viewerId: string, sessionId: string): void;
    get url(): string;
    get headers(): Headers;
    clear(): void;
    fetch(request: RequestParameters, variables: Variables): Promise<any>;
    toJSON(): {
        [key: string]: string;
    };
}
export default ServerFetcher;
