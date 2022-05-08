import React from 'react'
import { shallow } from 'enzyme'

import { UnwrappedUserNav as UserNav } from './index'

const props = {
  viewer: {},
}

describe('<UserNav />', () => {
  // @ts-expect-error
  const wrapper = shallow(<UserNav {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('includes a "Review" link', () => {
    const link = wrapper.find('Link')
    expect(link).toBeTruthy()
    expect(link.prop('to')).toEqual('/review')
  })
})
