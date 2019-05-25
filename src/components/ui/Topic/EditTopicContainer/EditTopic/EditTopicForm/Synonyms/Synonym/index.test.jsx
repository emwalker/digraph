import React from 'react'
import { shallow } from 'enzyme'

import { UnwrappedSynonym as Synonym } from './index'

const onDelete = jest.fn()

const props = {
  onDelete,
  synonym: {
    id: '1234',
    locale: 'en',
    name: 'Gnusto',
  },
}

describe('<Synonym />', () => {
  const wrapper = shallow(<Synonym {...props} />)
  const octicon = () => wrapper.find('Octicon')

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  describe('when onDelete is present', () => {
    beforeEach(() => {
      wrapper.setProps({ onDelete })
    })

    it('displays the trashcan icon', () => {
      expect(octicon().exists()).toBeTruthy()
    })
  })

  describe('when onDelete is not present', () => {
    beforeEach(() => {
      wrapper.setProps({ onDelete: null })
    })

    it('displays the trashcan icon', () => {
      expect(octicon().exists()).toBeFalsy()
    })
  })
})
