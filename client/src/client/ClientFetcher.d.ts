import { RequestParameters, Variables } from 'relay-runtime';
import { FetcherBase } from '../FetcherBase';
type PayloadsType = {
    [key: string]: string;
};
declare class ClientFetcher extends FetcherBase {
    constructor(payloads: PayloadsType);
    payloads: PayloadsType;
    get url(): string;
    fetch(request: RequestParameters, variables: Variables): Promise<any>;
}
export default ClientFetcher;
