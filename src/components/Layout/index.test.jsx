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

  it('does not include links for the moment', () => {
    expect(wrapper.find('.test-links-page')).toHaveLength(0)
  })
})
