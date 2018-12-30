// @flow
import React, { Component, type Node } from 'react'
import classNames from 'classnames'

import LinkOrA from './LinkOrA'
import type { Topic } from '../../types'
import TopicBadge from '../TopicBadge'

/* eslint no-underscore-dangle: 0 */

type Props = {
  children: Node,
  className: string,
  description?: ?string,
  displayColor: ?string,
  formIsOpen: boolean,
  newlyAdded: boolean,
  title: string,
  toggleForm: Function,
  topics: Topic[],
  url: ?string,
}

class Item extends Component<Props> {
  static defaultProps = {
    description: null,
    displayColor: '#fff',
  }

  get className() {
    return classNames(
      'Item-row',
      'Box-row',
      this.props.className,
      { 'anim-fade-in': this.props.newlyAdded },
    )
  }

  get style(): Object {
    return {
      borderLeft: `5px solid ${this.props.displayColor}`,
    }
  }

  get url(): ?Node {
    if (!this.props.url)
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
        <a
          className="Box-row-link"
          href="#"
        >
        {this.props.title}
        </a>
      )
    }

    return (
      <LinkOrA to={this.props.url} className="Box-row-link">
        { this.props.title }
      </LinkOrA>
    )
  }

  render() {
    const { formIsOpen, url } = this.props

    return (
      <li
        className={this.className}
        style={this.style}
        key={url}
      >
        <div className="d-flex flex-items-center">
          <div className="four-fifths">
            <div>
              {this.titleLink}
              <div>{ this.props.description }</div>
            </div>
            {this.url}
            <div>
              { this.props.topics.map(({ name, resourcePath }) => (
                <TopicBadge
                  key={resourcePath}
                  name={name}
                  resourcePath={resourcePath}
                />
              )) }
            </div>
          </div>
          <div className="one-fifth text-center">
            { !formIsOpen &&
              <button onClick={this.props.toggleForm} className="btn-link">Edit</button>
            }
          </div>
        </div>
        <div>
          { formIsOpen && this.props.children }
        </div>
      </li>
    )
  }
}

export default Item
