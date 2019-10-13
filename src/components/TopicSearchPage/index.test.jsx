import React from 'react'
import { shallow } from 'enzyme'

import TopicPage from './index'

jest.mock('react-relay', () => ({
  createFragmentContainer: (component) => component,
  createRefetchContainer: (Component) => (props) => (
    <Component {...props} relay={{ refetch: () => {} }} />
  ),
  QueryRenderer: () => null,
}))

describe('<TopicPage />', () => {
  const topic = {
    name: 'Frotz',
  }

  const view = {
    currentRepository: {
      displayName: 'Private collection',
      isPrivate: true,
    },
  }

  const wrapper = shallow(<TopicPage topic={topic} view={view} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
