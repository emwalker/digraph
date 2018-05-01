// @flow
import { compose, isNil, map, prop, propOr, reject } from 'ramda'

/* eslint import/prefer-default-export: 0 */

export const liftNodes = compose(reject(isNil), map(prop('node')), propOr([], 'edges'))
