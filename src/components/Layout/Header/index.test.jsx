import React from 'react'
import { shallow } from 'enzyme'

import Header from './index'

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

describe('<Header />', () => {
  const wrapper = shallow(<Header {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
