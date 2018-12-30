import React from 'react'
import { shallow } from 'enzyme'

import Item from './index'

const props = {
  children: 'child',
  className: '',
  formIsOpen: false,
  newlyAdded: false,
  loading: false,
  title: 'title',
  toggleForm: jest.fn(),
  topics: [],
  url: null,
}

describe('<Item />', () => {
  const wrapper = shallow(<Item {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  describe('loading', () => {
    const editLink = () => wrapper.find('button.btn-link')

    describe('when loading: true', () => {
      beforeEach(() => {
        wrapper.setProps({ loading: true })
      })

      it('hides the edit link', () => {
        expect(editLink().exists()).toBeFalsy()
      })
    })

    describe('when loading: false', () => {
      beforeEach(() => {
        wrapper.setProps({ loading: false })
      })

      it('hides the edit link', () => {
        expect(editLink().exists()).toBeTruthy()
      })
    })
  })
})
