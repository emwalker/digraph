import React from 'react'
import { shallow } from 'enzyme'

import FilterInput from './index'

const props = {
  onEnter: jest.fn(),
  location: {
    pathname: '/',
  },
  router: {
    push: () => {},
  },
}

describe('<FilterInput />', () => {
  const wrapper = shallow(<FilterInput {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
