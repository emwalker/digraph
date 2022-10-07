import React, { ReactNode } from 'react'
import classNames from 'classnames'

import { Color } from 'components/types'
import { topicPath } from 'components/helpers'
import LinkOrA from './LinkOrA'
import TopicBadge from '../TopicBadge'
import RepoOwnership from '../RepoOwnership'

/* eslint no-underscore-dangle: 0 */

type Topic = {
  displayName: string,
  id: string,
}

function locationDescriptor(pathname: string, itemTitle: string) {
  return {
    pathname,
    query: {},
    search: '',
    state: {
      itemTitle,
    },
  }
}

function CloseButton({ toggleForm }: { toggleForm: () => void }) {
  return (
    <dl className="form-group">
      <button className="btn-link" onClick={toggleForm} type="button">Close</button>
    </dl>
  )
}

function Url({ url, showLink }: { url: string | null, showLink: boolean }) {
  if (!url || !showLink) return null

  return (
    <div
      className="mt-1 link-url branch-name css-truncate css-truncate-target"
    >
      {url}
    </div>
  )
}

function TitleLink({ url, title }: { url: string | null, title: string }) {
  if (!url) {
    return (
      <a
        className="Box-row-link"
        href="#"
      >
        {title}
      </a>
    )
  }

  const to = locationDescriptor(url, title)

  return (
    <LinkOrA to={to} className="Box-row-link">
      {title}
    </LinkOrA>
  )
}

function OuterTopicBadge({ topic }: { topic: Topic }) {
  const { id, displayName } = topic

  return (
    <TopicBadge
      key={id}
      displayName={displayName}
      to={locationDescriptor(topicPath(id), displayName)}
    />
  )
}

type WideItemProps = {
  description: string | null,
  showLink: boolean,
  title: string,
  topics: Topic[],
  url: string | null,
}

function ItemView({ url, description, topics, title, showLink }: WideItemProps) {
  return (
    <div className="clearfix d-flex flex-items-center">
      <div className="col-12">
        <div>
          <TitleLink url={url} title={title} />
          <div>{description}</div>
        </div>
        <Url url={url} showLink={showLink} />
        <div>
          {topics.map((topic) => <OuterTopicBadge key={topic.id} topic={topic} />)}
        </div>
      </div>
    </div>
  )
}

type EditableItemProps = {
  children: ReactNode,
  description: string | null,
  formIsOpen: boolean,
  showEditButton: boolean,
  showLink: boolean,
  title: string,
  toggleForm: () => void,
  topics: Topic[],
  url: string | null,
}

function ItemEditForm({
  formIsOpen, description, url, topics, showEditButton, toggleForm, title, children, showLink,
}: EditableItemProps) {
  return (
    <div>
      <div className="clearfix d-flex flex-items-center">
        <div className="col-10">
          <div>
            <TitleLink url={url} title={title} />
            <div>{description}</div>
          </div>
          <Url url={url} showLink={showLink} />
          <div>
            {topics.map((topic) => <OuterTopicBadge key={topic.id} topic={topic} />)}
          </div>
        </div>
        <div className="col-2 text-center">
          {showEditButton
            ? (
              <button
                className="btn-link"
                onClick={toggleForm}
                type="button"
              >
                Edit
              </button>
            )
            : <a className="color-fg-muted no-underline">Edit</a>}
        </div>
      </div>
      <div>
        {formIsOpen && children}
      </div>
    </div>
  )
}

type Props = {
  canEdit: boolean,
  children: ReactNode,
  className: string,
  description?: string | null,
  formIsOpen: boolean,
  newlyAdded: boolean,
  repoColors: Color[],
  showEditButton: boolean,
  showLink?: boolean,
  showRepoOwnership: boolean,
  title: string,
  toggleForm: () => void,
  topics: Topic[],
  url: string | null,
}

export default function Item(props: Props) {
  const className = classNames('Item-row Box-row', props.className,
    { 'anim-fade-in': props.newlyAdded })
  const showEditButton = !props.formIsOpen && props.showEditButton === true

  const inner = props.canEdit
    ? (
      <ItemEditForm
        children={props.children}
        description={props.description || null}
        formIsOpen={props.formIsOpen}
        showEditButton={showEditButton}
        showLink={props.showLink || false}
        title={props.title}
        toggleForm={props.toggleForm}
        topics={props.topics}
        url={props.url}
      />
    )
    : (
      <ItemView
        description={props.description || null}
        showLink={props.showLink || false}
        title={props.title}
        topics={props.topics}
        url={props.url}
      />
    )

  return (
    <li className={className} key={props.url}>
      {inner}

      <div className="d-flex mb-1 mt-2 flex-items-center">
        <div className="mr-auto">
          <RepoOwnership
            showRepoOwnership={props.showRepoOwnership}
            repoColors={props.repoColors}
          />
        </div>

        {!showEditButton && <CloseButton toggleForm={props.toggleForm} />}
      </div>
    </li>
  )
}
