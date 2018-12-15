import React from 'react'
import { shallow } from 'enzyme'

import TopicPage from './index'

jest.mock('react-relay', () => ({ createFragmentContainer: component => component }))

describe('<TopicPage />', () => {
  const topic = {
    name: 'Frotz',
  }

  const view = {
    repository: {
      displayName: 'Private collection',
      isPrivate: true,
    },
  }

  const wrapper = shallow(
    <TopicPage
      topic={topic}
      view={view}
    />,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
