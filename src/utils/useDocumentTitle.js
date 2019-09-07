// @flow
import { useEffect } from 'react'

// From https://github.com/gaearon/react-document-title/issues/62#issuecomment-527554466

export default (title: string) => {
  useEffect(
    () => {
      const originalTitle = document.title
      document.title = title

      return () => {
        document.title = originalTitle
      }
    },
    [title],
  )
}
