import React from 'react'
import { shallow } from 'enzyme'

import FlashMessages from './index'

const props = {}

describe('<FlashMessages />', () => {
  const wrapper = shallow(<FlashMessages {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
