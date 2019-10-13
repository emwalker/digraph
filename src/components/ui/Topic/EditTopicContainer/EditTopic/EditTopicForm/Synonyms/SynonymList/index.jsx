// @flow
import React, { Component } from 'react'
import {
  SortableContainer,
  SortableElement,
  SortableHandle,
} from 'react-sortable-hoc'
import arrayMove from 'array-move'
import { GoThreeBars } from 'react-icons/go'

import Synonym from '../Synonym'
import { type Synonym as SynonymType } from '../types'
import copySynonyms from '../copySynonyms'

type Props = {
  canUpdate: boolean,
  onDelete: Function,
  onUpdate: Function,
  synonyms: $ReadOnlyArray<SynonymType>,
}

const DragHandle = SortableHandle(() => (
  <GoThreeBars
    className="synonym-drag-handle"
    style={{ width: '20px', height: '20px', color: '#d1d5da' }}
  />
))

const SortableSynonym = SortableElement((props) => <Synonym {...props} />)

type ContainerProps = {
  items: SynonymType[],
  onDelete: ?Function,
}

const SortableList = SortableContainer(({ onDelete, items }: ContainerProps) => (
  <div>
    {
      items.map((synonym, index) => (
        <SortableSynonym
          dragHandle={<DragHandle />}
          key={synonym.name}
          index={index}
          synonym={synonym}
          onDelete={onDelete}
        />
      ))
    }
  </div>
))

class SynonymList extends Component<Props> {
  onSortEnd = ({ oldIndex, newIndex }: { oldIndex: number, newIndex: number }) => {
    if (!this.props.canUpdate) return

    const synonyms = arrayMove(this.props.synonyms, oldIndex, newIndex)
    this.props.onUpdate(copySynonyms(synonyms))
  }

  get canSort(): boolean {
    return this.props.canUpdate && this.props.synonyms.length > 1
  }

  deleteFn = () => (
    this.props.canUpdate
      ? this.props.onDelete
      : null
  )

  renderReadonlyList = () => (
    // $FlowFixMe
    this.props.synonyms.map((value) => (
      <Synonym key={value.name} synonym={value} />
    ))
  )

  renderUpdatableList = () => (
    <SortableList
      items={this.props.synonyms}
      lockAxis="y"
      onDelete={this.deleteFn()}
      onSortEnd={this.onSortEnd}
      useDragHandle
    />
  )

  render = () => (
    this.canSort
      ? this.renderUpdatableList()
      : this.renderReadonlyList()
  )
}

export const UnwrappedSynonymList = SortableList

export default SynonymList
