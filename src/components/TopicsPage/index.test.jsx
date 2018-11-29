import React from 'react'
import { shallow } from 'enzyme'
import TopicsPage from './index'

jest.mock('react-relay', () => ({ createFragmentContainer: component => component }))

describe('<TopicsPage />', () => {
  const viewer = {
    name: 'Rezrov',
  }

  const view = {
    resourceId: 'organization:tyrell',
    topics: {
      edges: [
        {
          node: {
            name: 'Frotz',
            resourcePath: '/topics/1234',
          },
        },
      ],
    },
  }

  const wrapper = shallow(
    <TopicsPage
      view={view}
      viewer={viewer}
    />,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
