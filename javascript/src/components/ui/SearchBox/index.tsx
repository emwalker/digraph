import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import SearchBoxInner from './SearchBoxInner'

const SearchBox = (props: any) => <SearchBoxInner {...props} />

export default createFragmentContainer(SearchBox, {
  view: graphql`
    fragment SearchBox_view on View {
      queryInfo {
        stringTokens

        topics {
          edges {
            node {
              name
              resourcePath
            }
          }
        }
      }
    }
  `,
})
