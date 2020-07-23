// @flow
import { convertToRaw, EditorState } from 'draft-js'

class Query {
  data: Object

  constructor(editorState: EditorState) {
    this.data = convertToRaw(editorState.getCurrentContent())
  }

  get parts() {
    const buffer = []
    const { blocks, entityMap } = this.data

    blocks.forEach(({ text, entityRanges }) => {
      let lastStart = 0

      entityRanges.forEach(({ offset, length, key }) => {
        if (offset !== lastStart) buffer.push(text.slice(lastStart, offset))

        const { data: { mention } } = entityMap[key]

        if (mention !== undefined) {
          const { link } = mention
          buffer.push(`in:${link}`)
          lastStart = offset + length
        }
      })

      if (lastStart !== text.length) buffer.push(text.slice(lastStart))
    })

    return buffer
  }

  toString = () => this.parts.join('')
}

const queryFromState = (editorState: EditorState) => new Query(editorState)

export default queryFromState
