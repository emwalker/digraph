import React from 'react'
import { shallow } from 'enzyme'

import AddForm from './index'

jest.mock('react-relay', () => ({
  createFragmentContainer: (component) => component,
}))

const defaultViewer = {
  selectedRepository: {
    name: null,
    organization: {
      login: 'gnusto',
    },
  },
}

const props = {
  topic: {
    id: '1234',
  },
  viewer: defaultViewer,
}

describe('<AddForm />', () => {
  const wrapper = shallow(<AddForm {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  describe('selecting a repository', () => {
    const addLink = () => wrapper.find('AddLink')
    const addTopic = () => wrapper.find('AddTopic')

    describe('when a repository has been selected', () => {
      beforeEach(() => {
        wrapper.setProps({ viewer: defaultViewer })
      })

      it('enables the input fields', () => {
        expect(addLink().exists()).toBeTruthy()
        expect(addTopic().exists()).toBeTruthy()
      })
    })

    describe('when a repository has not yet been selected', () => {
      beforeEach(() => {
        wrapper.setProps({ viewer: { selectedRepository: null } })
      })

      it('disables the input fields', () => {
        expect(addLink().exists()).toBeFalsy()
        expect(addTopic().exists()).toBeFalsy()
      })
    })
  })
})
