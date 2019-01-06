import React from 'react'
import { shallow } from 'enzyme'

import LoadingPage from './index'

const props = {
  location: {
    pathname: '/some/path',
    state: {
      orgLogin: 'Gnusto',
      repoName: 'General collection',
      itemTitle: 'Some title',
    },
  },
}

describe('<LoadingPage />', () => {
  const wrapper = shallow(<LoadingPage {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
