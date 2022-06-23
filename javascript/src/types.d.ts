export declare type FoundRelayVariables = {
    topicId?: string;
    orgLogin?: string;
};
export declare type ReturnType<T extends (...args: any) => any> = T extends (...args: any) => infer R ? R : any;
