// @flow
import React, { Component } from 'react'
import type { Node } from 'react'
import classNames from 'classnames'

import type { Topic } from '../../types'
import TopicBadge from '../TopicBadge'

/* eslint no-underscore-dangle: 0 */

type Props = {
  children: Node,
  className: string,
  formIsOpen: boolean,
  title: string,
  toggleForm: Function,
  topics: Topic[],
  url: string,
}

class Item extends Component<Props> {
  get className() {
    return classNames(
      'Item-row',
      'Box-row',
      'Box-row--hover-gray',
      this.props.className,
    )
  }

  render() {
    const { formIsOpen, url } = this.props

    return (
      <li
        className={this.className}
        key={url}
      >
        <div className="d-flex flex-items-center">
          <div className="four-fifths">
            <div>
              <a className="Box-row-link" href={url}>
                { this.props.title }
              </a>
            </div>
            <div
              className="mt-1 link-url branch-name css-truncate css-truncate-target"
            >
              { url }
            </div>
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
