import React from 'react'
import {shallow} from 'enzyme'

import Header from './index'

const props = {
  viewer: {
    name: 'Frotz',
  },
}

describe('<Header />', () => {
  const wrapper = shallow(<Header {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
