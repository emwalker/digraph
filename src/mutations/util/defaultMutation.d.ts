import { GraphQLTaggedNode, Disposable } from 'react-relay';
import type { Updater } from './types';
declare type Mutator = (...args: any) => Disposable;
declare function defaultMutation<Input>(mutation: GraphQLTaggedNode, updater?: Updater): Mutator;
export default defaultMutation;
