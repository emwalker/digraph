import React, { Component, KeyboardEvent } from 'react'
import classNames from 'classnames'

import { everythingTopicPath } from 'components/constants'

type Props = {
  className?: string | undefined,
  router: {
    push: Function,
  },
}

class SearchBox extends Component<Props> {
  static defaultProps = {
    className: '',
  }

  onKeyPress = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      const { value } = event.currentTarget
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
        className="searchBoxInput form-control p-3"
        onKeyPress={this.onKeyPress}
        placeholder="Start with a search"
        type="search"
      />
    </p>
  )
}

export default SearchBox
