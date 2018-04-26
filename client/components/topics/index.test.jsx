import React from 'react'
import { shallow } from 'enzyme'
import Topics from './index'

jest.mock('react-relay', () => ({ createFragmentContainer: component => component }))

describe('<Topics />', () => {
  const viewer = {
    name: 'Rezrov',
  }

  const organization = {
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
    <Topics
      organization={organization}
      viewer={viewer}
      relay={relay}
    />,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('includes a form to add a topic', () => {
    expect(wrapper.find('.test-add-topic')).toHaveLength(1)
  })
})
