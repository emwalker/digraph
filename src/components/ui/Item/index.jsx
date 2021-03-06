// @flow
import React, { Component, type Node } from 'react'
import classNames from 'classnames'

import LinkOrA from './LinkOrA'
import TopicBadge from '../TopicBadge'
import { disabledLink } from './styles.module.css'

/* eslint no-underscore-dangle: 0 */

type Topic = {
  +displayName: string,
  +resourcePath: string,
}

type Props<T> = {
  canEdit: boolean,
  children: Node,
  className: string,
  description?: ?string,
  displayColor: ?string,
  formIsOpen: boolean,
  newlyAdded: boolean,
  showEditButton: ?boolean,
  showLink?: boolean,
  orgLogin: string,
  repoName: ?string,
  title: string,
  toggleForm: Function,
  topics: T[],
  url: ?string,
}

class Item<T: Topic> extends Component<Props<T>> {
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

  get style(): Object {
    if (!this.props.displayColor) return {}

    return {
      borderLeft: `5px solid ${this.props.displayColor}`,
    }
  }

  get url(): ?Node {
    if (!this.props.url || !this.props.showLink) return null

    return (
      <div
        className="mt-1 link-url branch-name css-truncate css-truncate-target"
      >
        { this.props.url }
      </div>
    )
  }

  get titleLink(): Node {
    if (!this.props.url) {
      return (
        // eslint-disable-next-line jsx-a11y/anchor-is-valid
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
        { this.props.title }
      </LinkOrA>
    )
  }

  locationDescriptor = (pathname: string, itemTitle: string) => (
    {
      pathname,
      state: {
        orgLogin: this.props.orgLogin,
        repoName: this.props.repoName,
        itemTitle,
      },
    }
  )

  renderTopicBadge = ({ displayName, resourcePath }: Topic) => (
    <TopicBadge
      key={resourcePath}
      displayName={displayName}
      to={this.locationDescriptor(resourcePath, displayName)}
    />
  )

  renderEditable = () => (
    <>
      <div className="clearfix d-flex flex-items-center">
        <div className="col-10">
          <div>
            { this.titleLink }
            <div>{ this.props.description }</div>
          </div>
          { this.url }
          <div>
            { this.props.topics.map(this.renderTopicBadge) }
          </div>
        </div>
        <div className="col-2 text-center">
          { this.showEditButton
            ? (
              <button
                className="btn-link"
                onClick={this.props.toggleForm}
                type="button"
              >
                Edit
              </button>
            )
            : <span className={disabledLink}>Edit</span>}
        </div>
      </div>
      <div>
        { this.props.formIsOpen && this.props.children }
      </div>
    </>
  )

  renderWide = () => (
    <div className="clearfix d-flex flex-items-center">
      <div className="col-12">
        <div>
          { this.titleLink }
          <div>{ this.props.description }</div>
        </div>
        { this.url }
        <div>
          { this.props.topics.map(this.renderTopicBadge) }
        </div>
      </div>
    </div>
  )

  render = () => {
    const { url } = this.props

    return (
      <li
        className={this.className}
        style={this.style}
        key={url}
      >
        { this.props.canEdit
          ? this.renderEditable()
          : this.renderWide()}
      </li>
    )
  }
}

export default Item
