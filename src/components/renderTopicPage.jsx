// @flow
import React from 'react'

import LoadingPage from 'components/LoadingPage'
import { defaultOrganizationLogin } from 'components/constants'
import TopicPage from './TopicPage'
import TopicSearchPage from './TopicSearchPage'

export default ({ props, error, match: { location } }: any) => {
  if (error)
    return <div>There was a problem.</div>

  if (!props)
    return <LoadingPage location={location} />

  if (!props.view)
    return <div>You must log in and select an organization first.</div>

  const { params, view } = props

  if (location.query.q) {
    return (
      <TopicSearchPage
        orgLogin={params.orgLogin || defaultOrganizationLogin}
        repoName={params.repoName}
        topic={view.topic}
        location={location}
        {...props}
      />
    )
  }

  return (
    <TopicPage
      location={location}
      orgLogin={params.orgLogin || defaultOrganizationLogin}
      repoName={params.repoName}
      topic={view.topic}
      {...props}
    />
  )
}
