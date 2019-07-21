// @flow
import React, { Component } from 'react'
import classNames from 'classnames'

import { everythingTopicPath } from 'components/constants'
import styles from './styles.module.css'

type Props = {
  className?: ?string,
  router: {
    push: Function,
  },
}

class SearchBox extends Component<Props> {
  static defaultProps = {
    className: '',
  }

  onKeyPress = (event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
    if (event.key === 'Enter') {
      const { value } = (event.target: window.HTMLInputElement)
      this.onSearch(value)
    }
  }

  onSearch = (query: string) => {
    if (query === '') {
      this.props.router.push({ pathname: everythingTopicPath })
      return
    }

    this.props.router.push({ pathname: everythingTopicPath, query: { q: query } })
  }

  render = () => (
    <p className={classNames(this.props.className, 'form-group text-center')}>
      <input
        aria-label="Search"
        className={classNames(styles.input, 'form-control p-3')}
        onKeyPress={this.onKeyPress}
        placeholder="Start with a search"
        type="search"
      />
    </p>
  )
}

export default SearchBox
