import type { CodegenConfig } from '@graphql-codegen/cli'

const config: CodegenConfig = {
  overwrite: true,
  schema: '../schema.graphql',
  documents: ['app/**/*.{ts,tsx}', 'lib/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
  generates: {
    'lib/__generated__/': {
      preset: 'client',
      plugins: [],
      config: {
        avoidOptionals: false,
      },
    },
  },
  ignoreNoDocuments: false,
}

export default config
