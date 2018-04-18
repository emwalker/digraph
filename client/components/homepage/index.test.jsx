import React from 'react'
import { shallow } from 'enzyme'
import Homepage from './index'

describe('<Homepage />', () => {
  const wrapper = shallow(<Homepage />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
