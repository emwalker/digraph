import React from 'react'
import { shallow } from 'enzyme'

import { UnwrappedSynonyms } from './index'

const synonymEdge = {
  node: {
    name: 'Gnusto',
    id: '1234',
  },
}

const topicDetail = {
  viewerCanDeleteSynonyms: true,
  viewerCanUpdate: true,
  synonyms: {
    edges: [synonymEdge],
  },
}

const props = {
  topicDetail,
}

describe('<Synonyms />', () => {
  // @ts-expect-error
  const wrapper = shallow(<UnwrappedSynonyms {...props} />)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })
})
