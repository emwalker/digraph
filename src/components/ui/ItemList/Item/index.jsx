// @flow
import React, { Component } from 'react'
import { pathOr } from 'ramda'
import classNames from 'classnames'

import EditLink from './EditLink'

/* eslint no-underscore-dangle: 0 */

const edges = pathOr([], ['edges'])

type BadgeProps = {
  node: {
    name: string,
    resourcePath: string,
  }
}

const renderTopic = ({ node: { name, resourcePath } }: BadgeProps) => (
  <a className="Box-row-link" href={resourcePath} key={resourcePath}>
    <span
      className="Label Label--outline Label--outline-gray"
    >
      {name}
    </span>
  </a>
)

type Props = {
  __typename: string,
  display: string,
  resourcePath: string,
  topics: Array[],
  FormComponent: Object,
}

type State = {
  formIsOpen: boolean,
}

class Item extends Component<Props, State> {
  state = {
    formIsOpen: false,
  }

  get className(): string {
    return classNames(
      'Item-row',
      'Box-row Box-row--hover-gray',
      { 'Box-row--topic': this.props.__typename === 'Topic' },
    )
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render() {
    return (
      <li
        className={this.className}
        key={this.props.resourcePath}
      >
        <div className="d-flex flex-items-center">
          <div className="four-fifths">
            <div>
              <a className="Box-row-link" href={this.props.resourcePath}>
                { this.props.display || this.props.resourcePath }
              </a>
            </div>
            <div
              className="mt-1 link-url branch-name css-truncate css-truncate-target"
            >
              {this.props.resourcePath}
            </div>
            <div>
              { edges(this.props.topics).map(renderTopic) }
            </div>
          </div>
          <div className="one-fifth text-center">
            {!this.state.formIsOpen &&
            <button onClick={this.toggleForm} className="btn-link">Edit</button>
            }
          </div>
        </div>
        <div>
          <this.props.FormComponent
            isOpen={this.state.formIsOpen}
            toggleFn={this.toggleForm}
            link={{}}
            {...this.props}
          />
        </div>
      </li>
    )
  }
}

export { EditLink }

export default Item
