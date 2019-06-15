import React from 'react'
import { shallow } from 'enzyme'

import { UnwrappedTopicPage as TopicPage } from './index'
import AddForm from './AddForm'

describe('<TopicPage />', () => {
  const topic = {
    displayName: 'Frotz',
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

    const form = () => wrapper.find(AddForm)

    it('shows the topic/link form', () => {
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
