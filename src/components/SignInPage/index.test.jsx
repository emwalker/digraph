import React from 'react'
import { shallow } from 'enzyme'

import SignInPage from './index'

const props = {}

describe('<SignInPage />', () => {
  const wrapper = shallow(<SignInPage {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
