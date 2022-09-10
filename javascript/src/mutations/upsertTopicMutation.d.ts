import { Environment } from 'react-relay';
import { DeclarativeMutationConfig } from 'react-relay';
import { UpsertTopicInput as Input } from '__generated__/upsertTopicMutation.graphql';
declare type Config = {
    configs: DeclarativeMutationConfig[];
};
declare const _default: (environment: Environment, input: Input, config: Config) => import("react-relay").Disposable;
export default _default;
