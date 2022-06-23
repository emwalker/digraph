export type FoundRelayVariables = {
  topicId?: string,
  orgLogin?: string,
}

export type ReturnType<
  T extends (...args: any) => any,
  > = T extends (...args: any) => infer R
    ? R
    : any;
