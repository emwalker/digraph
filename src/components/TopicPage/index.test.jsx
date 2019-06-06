import React from 'react'
import { shallow } from 'enzyme'

import TopicPage from './index'

jest.mock('react-relay', () => ({
  createFragmentContainer: component => component,
  createRefetchContainer: Component => props => (
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

  const viewer = {
    isGuest: false,
  }

  const wrapper = shallow(
    <TopicPage
      topic={topic}
      view={view}
      viewer={viewer}
    />,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  describe('when the viewer is logged in', () => {
    beforeEach(() => {
      wrapper.setProps({ viewer: { ...viewer, isGuest: false } })
    })

    const form = () => wrapper.find('AddForm')

    it('hides the topic/link form', () => {
      expect(form().exists()).toBeTruthy()
    })
  })

  describe('when the viewer is a guest', () => {
    beforeEach(() => {
      wrapper.setProps({ viewer: { ...viewer, isGuest: true } })
    })

    const form = () => wrapper.find('AddForm')

    it('hides the topic/link form', () => {
      expect(form().exists()).toBeFalsy()
    })
  })
})
