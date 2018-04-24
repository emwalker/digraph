import React from 'react'
import { shallow } from 'enzyme'
import Topic from './index'

describe('<Topic />', () => {
  const topic = {
    name: 'Frotz',
    resourceId: '/topics/1234',
    id: '1234',
  }

  const wrapper = shallow(<Topic topic={topic} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
