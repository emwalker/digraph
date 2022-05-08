export type FoundRelayVariables = {}

export type ReturnType<
  T extends (...args: any) => any,
  > = T extends (...args: any) => infer R
    ? R
    : any;
