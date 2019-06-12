import React from 'react'
import { shallow } from 'enzyme'

import SynonymList from './index'

const synonym = {
  name: 'Gnusto',
  id: '1234',
}

const props = {
  canUpdate: true,
  synonyms: [synonym],
  onDelete: jest.fn(),
  topic: {
    name: 'Gnusto',
    id: '1234',
  },
}

describe('<SynonymList />', () => {
  const wrapper = shallow(<SynonymList {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
