import React from 'react'
import { shallow } from 'enzyme'

import TopicPage from './index'

jest.mock('react-relay', () => ({
  createFragmentContainer: (component: any) => component,
  createRefetchContainer: (Component: any) => (props: any) => (
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

  // @ts-expect-error
  const wrapper = shallow(<TopicPage topic={topic} view={view} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
