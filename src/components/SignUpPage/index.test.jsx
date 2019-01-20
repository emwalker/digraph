import React from 'react'
import { shallow } from 'enzyme'

import SignUpPage from './index'

const props = {}

describe('<SignUpPage />', () => {
  const wrapper = shallow(<SignUpPage {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
