import React from 'react'
import { shallow } from 'enzyme'
import AddTopic from './index'

jest.mock('react-relay', () => ({ createFragmentContainer: component => component }))

describe('<AddTopic />', () => {
  const wrapper = shallow(
    <AddTopic />,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('includes an input for the name', () => {
    expect(wrapper.find('input').hasClass('test-topic-name')).toBeTruthy()
  })

  it('includes a tooltip', () => {
    expect(wrapper.find('.tooltipped').exists()).toBeTruthy()
  })
})
