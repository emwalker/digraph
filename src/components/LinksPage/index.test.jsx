import React from 'react'
import { shallow } from 'enzyme'
import LinksPage from './index'

jest.mock('react-relay', () => ({ createFragmentContainer: component => component }))

describe('<LinksPage />', () => {
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
            resourceId: '/topics/1234',
            id: '1234',
          },
        },
      ],
    },
  }

  const relay = {
    environment: {},
  }

  const wrapper = shallow(
    <LinksPage
      view={view}
      viewer={viewer}
      relay={relay}
    />,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
