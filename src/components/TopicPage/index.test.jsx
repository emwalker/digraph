import React from 'react'
import { shallow } from 'enzyme'

import TopicPage from './index'

jest.mock('react-relay', () => ({ createFragmentContainer: component => component }))

describe('<TopicPage />', () => {
  const topic = {
    name: 'Frotz',
  }

  const wrapper = shallow(<TopicPage topic={topic} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
