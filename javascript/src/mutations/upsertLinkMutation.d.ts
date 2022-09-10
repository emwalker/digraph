import { Environment } from 'react-relay';
import { DeclarativeMutationConfig } from 'react-relay';
import type { UpsertLinkInput } from '__generated__/upsertLinkMutation.graphql';
export declare type Input = UpsertLinkInput;
declare type Config = {
    configs: DeclarativeMutationConfig[];
};
declare const _default: (environment: Environment, input: Input, config?: Config) => import("react-relay").Disposable;
export default _default;
