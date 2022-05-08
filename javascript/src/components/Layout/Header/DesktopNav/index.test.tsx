import React from 'react'
import { shallow } from 'enzyme'

import { UnwrappedDesktopNav as DesktopNav } from './index'

const props = {
  viewer: {
    name: 'Frotz',
    defaultRepository: {
      rootTopic: {
        resourcePath: '/some-repo/topics/123',
      },
    },
  },
}

describe('<DesktopNav />', () => {
  // @ts-expect-error
  const wrapper = shallow(<DesktopNav {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
