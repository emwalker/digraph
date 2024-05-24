export interface FoundRelayVariables {
    repoIds?: readonly string[] | null | undefined;
    searchString: string;
    topicId: string;
    viewerId: string;
}
export type ReturnType<T extends (...args: any) => any> = T extends (...args: any) => infer R ? R : any;
