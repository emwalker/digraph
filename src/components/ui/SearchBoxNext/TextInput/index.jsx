// @flow
import React, { useState, useCallback } from 'react'
import { ContentState, EditorState } from 'draft-js'
import Editor from 'draft-js-plugins-editor'
import createMentionPlugin from 'draft-js-mention-plugin'
import createSingleLinePlugin from 'draft-js-single-line-plugin'
import 'draft-js-mention-plugin/lib/plugin.css'

import styles from './styles.module.css'
import TopicSuggestions from './TopicSuggestions'

const mentionPlugin = createMentionPlugin({
  mentionPrefix: '',
  mentionTrigger: 'in:',
  supportWhitespace: true,
  mentionComponent: (mentionProps) => (
    <span className="Label mr-1 Label--gray-darker">
      { mentionProps.children }
    </span>
  ),
})

const singleLinePlugin = createSingleLinePlugin({ stripEntities: false })
const { MentionSuggestions } = mentionPlugin
const plugins = [mentionPlugin, singleLinePlugin]

const stateFor = (text: string) => EditorState.createWithContent(ContentState.createFromText(text || ''))

type Props = {
  defaultValue: string,
  handleReturn: Function,
}

const TextInput = ({ defaultValue, handleReturn }: Props) => {
  const [editorState, setEditorState] = useState(stateFor(defaultValue))
  const [mentionListOpen, setMentionListOpen] = useState(false)
  const [hasFocus, setHasFocus] = useState(false)

  const wrappedHandleReturn = useCallback((event, nextEditorState) => {
    if (!mentionListOpen) handleReturn(event, nextEditorState)
  }, [mentionListOpen])

  const onFocus = useCallback(() => setHasFocus(true), [setHasFocus])
  const onBlur = useCallback(() => setHasFocus(false), [setHasFocus])
  const focus = hasFocus ? 'focus' : ''

  return (
    <div className={`${styles.textInput} form-control ${focus}`}>
      <Editor
        editorState={editorState}
        handleReturn={wrappedHandleReturn}
        onBlur={onBlur}
        onChange={setEditorState}
        onFocus={onFocus}
        placeholder="Search"
        plugins={plugins}
      />
      <TopicSuggestions
        Wrapped={MentionSuggestions}
        setMentionListOpen={setMentionListOpen}
      />
    </div>
  )
}

export default TextInput
