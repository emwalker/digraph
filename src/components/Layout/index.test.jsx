import React from 'react'
import { shallow } from 'enzyme'
import Layout from './index'

describe('<Layout />', () => {
  const wrapper = shallow(
    <Layout>
      <div>some view</div>
    </Layout>,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('includes a topics menu item', () => {
    expect(wrapper.find('.test-topics-page')).toHaveLength(1)
  })

  it('includes a links menu item', () => {
    expect(wrapper.find('.test-links-page')).toHaveLength(1)
  })
})
