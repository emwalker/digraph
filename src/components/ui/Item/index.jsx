// @flow
import React, { Component, Fragment, type Node } from 'react'
import classNames from 'classnames'

import type { TopicType } from 'components/types'
import LinkOrA from './LinkOrA'
import TopicBadge from '../TopicBadge'

/* eslint no-underscore-dangle: 0 */

type Props = {
  canEdit: boolean,
  children: Node,
  className: string,
  description?: ?string,
  displayColor: ?string,
  formIsOpen: boolean,
  newlyAdded: boolean,
  showEditButton: ?boolean,
  showLink: ?boolean,
  orgLogin: string,
  repoName: ?string,
  title: string,
  toggleForm: Function,
  topics: TopicType[],
  url: ?string,
}

class Item extends Component<Props> {
  static defaultProps = {
    description: null,
    displayColor: '#fff',
    showEditButton: false,
    showLink: true,
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
    if (!this.props.displayColor)
      return {}

    return {
      borderLeft: `5px solid ${this.props.displayColor}`,
    }
  }

  get url(): ?Node {
    if (!this.props.url || !this.props.showLink)
      return null

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

  renderTopicBadge = (
    // eslint-disable-next-line react/no-unused-prop-types
    { name, resourcePath }: {name: string, resourcePath: string},
  ) => (
    <TopicBadge
      key={resourcePath}
      name={name}
      to={this.locationDescriptor(resourcePath, name)}
    />
  )

  renderEditable = () => (
    <Fragment>
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
          { this.showEditButton &&
            <button onClick={this.props.toggleForm} className="btn-link">Edit</button>
          }
        </div>
      </div>
      <div>
        { this.props.formIsOpen && this.props.children }
      </div>
    </Fragment>
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
          : this.renderWide()
        }
      </li>
    )
  }
}

export default Item
