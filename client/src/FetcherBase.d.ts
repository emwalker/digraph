import 'isomorphic-fetch';
import { RequestParameters, Variables } from 'relay-runtime';
export declare class FetcherBase {
    private _url;
    get url(): string;
    set url(value: string);
    get headers(): {
        'Content-Type': string;
    };
    fetch(request: RequestParameters, variables: Variables): Promise<any>;
}
export default FetcherBase;
