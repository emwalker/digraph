import React from 'react'
import { shallow } from 'enzyme'
import AddTopic from './index'

describe('<AddTopic />', () => {
  const organization = {
    resourceId: 'organization:tyrell',
  }

  const wrapper = shallow(
    <AddTopic
      organization={organization}
    />,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('includes an input for the name', () => {
    expect(wrapper.find('input.test-topic-name')).toHaveLength(1)
  })
})
