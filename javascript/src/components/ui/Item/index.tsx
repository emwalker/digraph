import React, { Component, ReactNode } from 'react'
import classNames from 'classnames'

import { Color } from 'components/types'
import { topicPath } from 'components/helpers'
import { LocationType } from 'components/types'
import LinkOrA from './LinkOrA'
import TopicBadge from '../TopicBadge'
import RepoOwnership from '../RepoOwnership'

/* eslint no-underscore-dangle: 0 */

type Topic = {
  displayName: string,
  id: string,
} | null

type Props = {
  canEdit: boolean,
  children: ReactNode,
  className: string,
  description?: string | null,
  formIsOpen: boolean,
  newlyAdded: boolean,
  repoColors: Color[],
  showEditButton: boolean | null,
  showLink?: boolean,
  showRepoOwnership: boolean,
  title: string,
  toggleForm: () => void,
  topics: Topic[],
  url: string | null,
}

class Item extends Component<Props> {
  static defaultProps = {
    description: null,
    showLink: false,
  }

  get className(): string {
    return classNames(
      'Item-row',
      'Box-row',
      this.props.className,
      { 'anim-fade-in': this.props.newlyAdded },
    )
  }

  get showEditButton(): boolean {
    return !this.props.formIsOpen && this.props.showEditButton === true
  }

  get url() {
    if (!this.props.url || !this.props.showLink) return null

    return (
      <div
        className="mt-1 link-url branch-name css-truncate css-truncate-target"
      >
        {this.props.url}
      </div>
    )
  }

  get titleLink() {
    if (!this.props.url) {
      return (
        <a
          className="Box-row-link"
          href="#"
        >
          {this.props.title}
        </a>
      )
    }

    const to = this.locationDescriptor(this.props.url, this.props.title)

    return (
      <LinkOrA to={to} className="Box-row-link">
        {this.props.title}
      </LinkOrA>
    )
  }

  locationDescriptor = (pathname: string, itemTitle: string): LocationType => (
    {
      pathname,
      query: {},
      search: '',
      state: {
        itemTitle,
      },
    }
  )

  renderTopicBadge = (topic: Topic) => {
    if (!topic) return null
    const { id, displayName } = topic
    return (
      <TopicBadge
        key={id}
        displayName={displayName}
        to={this.locationDescriptor(topicPath(id), displayName)}
      />
    )
  }

  renderEditable = () => (
    <>
      <div className="clearfix d-flex flex-items-center">
        <div className="col-10">
          <div>
            {this.titleLink}
            <div>{this.props.description}</div>
          </div>
          {this.url}
          <div>
            {this.props.topics.map(this.renderTopicBadge)}
          </div>
        </div>
        <div className="col-2 text-center">
          {this.showEditButton
            ? (
              <button
                className="btn-link"
                onClick={this.props.toggleForm}
                type="button"
              >
                Edit
              </button>
            )
            : <span className="itemDisabledLink">Edit</span>}
        </div>
      </div>
      <div>
        {this.props.formIsOpen && this.props.children}
      </div>
    </>
  )

  renderWide = () => (
    <div className="clearfix d-flex flex-items-center">
      <div className="col-12">
        <div>
          {this.titleLink}
          <div>{this.props.description}</div>
        </div>
        {this.url}
        <div>
          {this.props.topics.map(this.renderTopicBadge)}
        </div>
      </div>
    </div>
  )

  render = () => (
    <li className={this.className} key={this.props.url}>
      {this.props.canEdit
        ? this.renderEditable()
        : this.renderWide()}

      <RepoOwnership
        showRepoOwnership={this.props.showRepoOwnership}
        repoColors={this.props.repoColors}
      />
    </li>
  )
}

export default Item
