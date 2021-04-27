import React from 'react'
import { shallow } from 'enzyme'

import { UnwrappedLink as Link } from './index'
import Item from '../Item'

const props = {
  link: {
    title: 'title',
    url: 'url',
  },
  orgLogin: 'gnusto',
  relay: {},
  view: {
    currentRepository: {
      name: 'General collection',
    },
  },
  viewer: {},
}

describe('<Link />', () => {
  // @ts-expect-error
  const wrapper = shallow(<Link {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('passes the showLink flag to the Item component', () => {
    const item = wrapper.find(Item)
    expect(item.prop('showLink')).toBeTruthy()
  })
})
