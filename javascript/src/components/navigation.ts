/* eslint import/prefer-default-export: 0 */

import { everythingTopicId } from './constants'
import { topicPath } from './helpers'

export const toEverything = {
  pathname: topicPath(everythingTopicId),
  state: {
    itemTitle: 'Everything',
  },
}
