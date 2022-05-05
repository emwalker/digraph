import React, { useState, useCallback, useRef, KeyboardEvent, ReactNode } from 'react'
import { EditorState, convertFromRaw, genKey, DraftHandleValue } from 'draft-js'
import Editor from '@draft-js-plugins/editor'
import createSingleLinePlugin from 'draft-js-single-line-plugin'
import createMentionPlugin from '@draft-js-plugins/mention'

import { SearchBox_view as ViewType } from '__generated__/SearchBox_view.graphql'
import rawFromQuery from './rawFromQuery'
import styles from './styles.module.css'
import TopicSuggestions from './TopicSuggestions'

require('@draft-js-plugins/mention/lib/plugin.css')

type ReturnHandler = (e: KeyboardEvent, editorState: EditorState) => DraftHandleValue
type QueryInfo = ViewType['queryInfo']

const mentionPlugin = createMentionPlugin({
  mentionPrefix: '',
  mentionTrigger: 'in:',
  supportWhitespace: true,
  mentionComponent: (mentionProps: { children: ReactNode }) => (
    <span className="Label mr-1">
      { mentionProps.children }
    </span>
  ),
})

const singleLinePlugin = createSingleLinePlugin({ stripEntities: false })
const { MentionSuggestions } = mentionPlugin
const plugins = [mentionPlugin, singleLinePlugin]

const stateFor = (queryInfo: QueryInfo) => {
  if (queryInfo == null) return EditorState.createEmpty()
  return EditorState.createWithContent(convertFromRaw(rawFromQuery(queryInfo, genKey)))
}

type Props = {
  handleReturn: ReturnHandler,
  queryInfo: QueryInfo,
}

const TextInput = ({ handleReturn, queryInfo }: Props) => {
  const [editorState, setEditorState] = useState(stateFor(queryInfo))
  const [mentionListOpen, setMentionListOpen] = useState(false)
  const [hasFocus, setHasFocus] = useState(false)
  const editor = useRef(null)

  const wrappedHandleReturn = useCallback((event: KeyboardEvent, nextEditorState: EditorState) => {
    if (!mentionListOpen) {
      handleReturn(event, nextEditorState)
      return 'handled'
    }
    return 'not-handled'
  }, [mentionListOpen])

  const onFocus = useCallback(() => setHasFocus(true), [setHasFocus])
  const onBlur = useCallback(() => setHasFocus(false), [setHasFocus])
  const focus = hasFocus ? 'focus' : ''

  return (
    <div className={`${styles.textInput} form-control ${focus}`}>
      <Editor
        ref={editor}
        editorState={editorState}
        handleReturn={wrappedHandleReturn}
        onBlur={onBlur}
        onChange={setEditorState}
        onFocus={onFocus}
        placeholder="Search"
        plugins={plugins}
        stripPastedStyles
      />
      <TopicSuggestions
        Suggestions={MentionSuggestions}
        setMentionListOpen={setMentionListOpen}
        isOpen={mentionListOpen}
      />
    </div>
  )
}

export default TextInput
