import React from 'react'
import { shallow } from 'enzyme'

import AddLink from './index'

jest.mock('react-relay', () => ({ createFragmentContainer: (component: any) => component }))

const props = {
  disabled: false,
  relay: { environment: {} },
  topic: {},
  viewer: {},
}

describe('<AddLink />', () => {
  // @ts-expect-error
  const wrapper = shallow(<AddLink {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('has a tooltip', () => {
    expect(wrapper.find('.tooltipped').exists()).toBeTruthy()
  })
})
