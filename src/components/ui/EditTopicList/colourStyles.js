// @flow
import chroma from 'chroma-js'

type Option = {|
  data: {
    color: string,
  },
  isDisabled?: boolean,
  isFocused?: boolean,
  isSelected?: boolean,
|}

type Styles = {||}

/* eslint no-nested-ternary: 0 */

export default {
  control: (styles: Styles) => ({ ...styles, backgroundColor: 'white' }),

  option: (styles: Styles, {
    data, isDisabled, isFocused, isSelected,
  }: Option) => {
    const color = chroma(data.color)
    return {
      ...styles,
      backgroundColor: isDisabled
        ? null
        : isSelected ? data.color : isFocused ? color.alpha(0.1).css() : null,
      color: isDisabled
        ? '#ccc'
        : isSelected
          ? chroma.contrast(color, 'white') > 2 ? 'white' : 'black'
          : data.color,
      cursor: isDisabled ? 'not-allowed' : 'default',
    }
  },

  multiValue: (styles: Styles, { data }: Option) => {
    const color = chroma(data.color)
    return {
      ...styles,
      backgroundColor: color.alpha(0.1).css(),
    }
  },

  multiValueLabel: (styles: Styles, { data }: Option) => ({
    ...styles,
    color: data.color,
  }),

  multiValueRemove: (styles: Styles, { data }: Option) => ({
    ...styles,
    color: data.color,
    ':hover': {
      backgroundColor: data.color,
      color: 'white',
    },
  }),
}
