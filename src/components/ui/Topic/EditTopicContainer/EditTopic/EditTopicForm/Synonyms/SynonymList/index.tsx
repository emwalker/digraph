import React, { Component } from 'react'
import {
  SortableContainer,
  SortableElement,
  SortableHandle,
} from 'react-sortable-hoc'
import arrayMove from 'array-move'
import { GoThreeBars } from 'react-icons/go'

import { Synonyms_topic as TopicType } from '__generated__/Synonyms_topic.graphql'
import Synonym from '../Synonym'
import copySynonyms from '../copySynonyms'

type SynonymType = TopicType['synonyms'][number]

type Props = {
  canUpdate: boolean,
  onDelete: Function,
  onUpdate: Function,
  synonyms: readonly SynonymType[],
}

const DragHandle = SortableHandle(() => (
  <GoThreeBars
    className="synonym-drag-handle"
    style={{ width: '20px', height: '20px', color: '#d1d5da' }}
  />
))

const SortableSynonym = SortableElement((props: any) => <Synonym {...props} />)

type ContainerProps = {
  items: readonly SynonymType[],
  onDelete: Function | null,
}

const SortableList = SortableContainer(({ onDelete, items }: ContainerProps) => (
  <div>
    {
      items.map((synonym, index) => (
        <SortableSynonym
          // @ts-ignore
          dragHandle={<DragHandle />}
          index={index}
          key={synonym.name}
          onDelete={onDelete}
          position={index}
          synonym={synonym}
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
    this.props.synonyms.map((value) => (
      <Synonym key={value.name} synonym={value} />
    ))
  )

  renderUpdatableList = () => (
    <SortableList
      // @ts-ignore
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
