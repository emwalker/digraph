import React from 'react'
import { shallow } from 'enzyme'
import LinkList from './index'

jest.mock('react-relay', () => ({ createFragmentContainer: component => component }))

describe('<LinkList />', () => {
  const viewer = {
    name: 'Rezrov',
  }

  const organization = {}

  const relay = {
    environment: {},
  }

  const wrapper = shallow(
    <LinkList
      organization={organization}
      relay={relay}
      title="Frotz"
      viewer={viewer}
    />,
  )

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  it('includes a form to add a link', () => {
    expect(wrapper.find('.test-add-link')).toHaveLength(1)
  })
})
