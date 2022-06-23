import React from 'react'
import { isEmpty } from 'ramda'
import { Link } from 'found'

import BlankslateUI from '../Blankslate'

const Blankslate = ({ placeholder }: { placeholder: string }) => (
  <BlankslateUI>
    <p>{ placeholder }</p>
  </BlankslateUI>
)

type ItemType = {
  display: string,
  path: string,
} | null

type ItemListProps = {
  items: ItemType[],
  orgLogin: string,
  repoName: string,
}

const renderItem = (orgLogin: string, repoName: string) => (item: ItemType) => {
  if (!item) return null
  const { path, display } = item

  const to = {
    pathname: path,
    state: {
      itemTitle: display,
      orgLogin,
      repoName,
    },
  }

  return (
    <li
      className="Box-row"
      key={path}
    >
      <Link to={to} className="Box-row-link">
        { display }
      </Link>
    </li>
  )
}

const ItemList = ({ items, orgLogin, repoName }: ItemListProps) => {
  const render = renderItem(orgLogin, repoName)
  return (
    <ul>
      { items.map(render) }
    </ul>
  )
}

type Props = {
  items: ItemType[],
  orgLogin: string,
  placeholder: string,
  repoName: string,
  title: string,
}

export default ({ items, orgLogin, placeholder, repoName, title }: Props) => (
  <div className="Box Box--condensed mb-3">
    <div className="Box-header">
      <span className="Box-title">{title}</span>
    </div>
    { isEmpty(items)
      ? <Blankslate placeholder={placeholder} />
      : <ItemList items={items} orgLogin={orgLogin} repoName={repoName} />}
  </div>
)
