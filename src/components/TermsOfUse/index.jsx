// @flow
import React from 'react'
import Markdown from 'react-markdown'

import Page from 'components/ui/Page'

const terms = `
Content added to Digraph's public repository is intended to be shared with others without
restriction.

* There are two repositories of links and topics that each user has access to.
* Content added to the public repository ("wiki") will be visible to everyone.  By adding links
  and topics to the public repo, you are making them available to others without restriction,
  and the content will outlive your account if you decide to delete it.
* Links and topics added to your private repo are yours and will not be visible to other people.
  Content in your private repo will be removed from the site when you delete your account (but may be
  retained in periodic database backups).
`

export default () => (
  <Page>
    <div className="pagehead">
      <h1>Terms of use</h1>
    </div>

    <div className="markdown-body mt-3">
      <Markdown>{terms}</Markdown>
    </div>
  </Page>
)
