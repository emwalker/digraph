import React from 'react'
import { shallow } from 'enzyme'
import Layout from './index'

describe('<Layout />', () => {
  const router = {
    push: jest.fn(),
  }

  const location = {
    query: { q: '' },
    search: '',
  }

  const view = {
    viewer: {},
  }

  const wrapper = shallow(
    <Layout
      router={router}
      location={location}
      view={view}
    >
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
