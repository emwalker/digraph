// @flow
import {
  compose,
  isNil,
  map,
  prop,
  propOr,
  reject,
  uniq,
} from 'ramda'

/* eslint import/prefer-default-export: 0 */

export const liftNodes = compose(
  uniq,
  reject(isNil),
  map(prop('node')),
  propOr([], 'edges'),
)
