import React from 'react'
import { shallow } from 'enzyme'

import { UnwrappedHeader as Header } from './index'

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
  const recentActivity = () => wrapper.find('#recent-activity')

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('includes a "Recent" link', () => {
    expect(recentActivity().exists()).toBeTruthy()
  })
})
