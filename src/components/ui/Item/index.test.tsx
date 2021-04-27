import React from 'react'
import { shallow } from 'enzyme'

import Item from './index'

const props = {
  canEdit: true,
  children: 'child',
  className: '',
  formIsOpen: false,
  showEditButton: false,
  loading: false,
  title: 'title',
  toggleForm: jest.fn(),
  topics: [],
  url: null,
}

describe('<Item />', () => {
  // @ts-expect-error
  const wrapper = shallow(<Item {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  describe('showEditButton', () => {
    const editButton = () => wrapper.find('button.btn-link')

    describe('when showEditButton: true', () => {
      beforeEach(() => {
        wrapper.setProps({ showEditButton: true })
      })

      it('hides the edit link', () => {
        expect(editButton().exists()).toBeTruthy()
      })
    })

    describe('when showEditButton: false', () => {
      beforeEach(() => {
        wrapper.setProps({ showEditButton: false })
      })

      it('hides the edit link', () => {
        expect(editButton().exists()).toBeFalsy()
      })
    })
  })
})
