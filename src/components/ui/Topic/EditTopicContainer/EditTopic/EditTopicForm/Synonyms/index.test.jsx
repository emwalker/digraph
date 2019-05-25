import React from 'react'
import { shallow } from 'enzyme'

import { UnwrappedSynonyms } from './index'
import Synonym from './Synonym'

const synonymEdge = {
  node: {
    name: 'Gnusto',
    id: '1234',
  },
}

const topic = {
  viewerCanDeleteSynonyms: true,
  synonyms: {
    edges: [synonymEdge],
  },
}

const props = {
  topic,
}

describe('<Synonyms />', () => {
  const wrapper = shallow(<UnwrappedSynonyms {...props} />)
  const rows = () => wrapper.find(Synonym)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  describe('when viewerCanDeleteSynonyms: false', () => {
    beforeEach(() => {
      wrapper.setProps({ topic: { ...topic, viewerCanDeleteSynonym: false } })
    })

    it('does not provide an onDelete handler', () => {
      expect(rows().at(0).prop('onDelete')).toBeFalsy()
    })
  })

  describe('when viewerCanDeleteSynonyms: true', () => {
    beforeEach(() => {
      wrapper.setProps({ topic: { ...topic, viewerCanDeleteSynonym: true } })
    })

    it('provides an onDelete handler', () => {
      expect(rows().at(0).prop('onDelete')).toBeTruthy()
    })
  })
})
