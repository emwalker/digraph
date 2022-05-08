import React from 'react'
import { shallow } from 'enzyme'

import Alert from './index'

const props = {
  message: 'Gnusto',
}

describe('<Alert />', () => {
  // @ts-expect-error
  const wrapper = shallow(<Alert {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
