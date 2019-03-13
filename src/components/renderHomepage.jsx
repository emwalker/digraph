// @flow
import React from 'react'
import Homepage from 'components/Homepage'
import { isMobile } from 'react-device-detect'
import RedirectException from 'found/lib/RedirectException'

import { everythingTopicPath } from 'components/constants'

const isChrome = Boolean(window.chrome)

type Props = {
  props: ?Object,
}

export default ({ props }: Props) => {
  if (isChrome && isMobile)
    throw new RedirectException(everythingTopicPath)

  return <Homepage {...props} />
}
