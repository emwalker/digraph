import React from 'react'
import { shallow } from 'enzyme'

import TermsOfUse from '.'

const props = {}

describe('<TermsOfUse />', () => {
  const wrapper = shallow(<TermsOfUse {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
