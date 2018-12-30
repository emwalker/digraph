import React from 'react'
import { shallow } from 'enzyme'

import AddForm from './index'

const props = {
  topic: {
    id: '1234',
  },
  viewer: {
    selectedRepository: {
      name: null,
      organization: {
        login: 'gnusto',
      },
    },
  },
}

describe('<AddForm />', () => {
  const wrapper = shallow(<AddForm {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
