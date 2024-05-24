'use client'

import { CSSVariablesResolver, createTheme } from '@mantine/core'

export const cssVariablesResolver: CSSVariablesResolver = () => ({
  variables: {},
  light: {},
  dark: {},
})

export const theme = createTheme({
  primaryColor: 'blue',
  colors: {
    dark: [
      '#C9C9C9',
      '#B8B8B8',
      '#575E69',
      '#444C58',
      '#313946',
      '#1e2734',
      '#161F2C',
      '#101722',
      '#0A0F18',
      '#04080D',
      '#000001',
    ],
  },
})
