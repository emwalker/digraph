import React from 'react'
import { shallow } from 'enzyme'

import Footer from '.'

const props = {}

describe('<Footer />', () => {
  const wrapper = shallow(<Footer {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('has a link to the terms of use', () => {
    expect(wrapper.find('[test-id="terms-of-use"]').exists()).toBeTruthy()
  })
})
