module.exports = {
  extends: 'airbnb-typescript',

  rules: {
    curly: ['error', 'multi-or-nest', 'consistent'],
    'jsx-a11y/anchor-is-valid': 'off',
    'jsx-a11y/label-has-associated-control': 'off',
    'jsx-a11y/label-has-for': 'off',
    // Needed for semi: ["error", never"]
    'no-unexpected-multiline': 'error',
    'object-curly-newline': ['error', { consistent: true }],
    'react/destructuring-assignment': 'off',
    'react/jsx-props-no-spreading': 'off',
    'react/require-default-props': 'off',
    'react/static-property-placement': ['error', 'static public field'],
    semi: ['error', 'never'],
    'template-curly-spacing': 'off',
    indent: 'off',

    // Typescript
    '@typescript-eslint/semi': 'off',
  },

  overrides: [
    {
      files: ['**/*.test.jsx'],
    },
  ],

  parser: '@typescript-eslint/parser',
  parserOptions: {
    project: `./tsconfig.json`
  },

  env: {
    browser: true,
    jest: true,
  },

  plugins: [
    'react',
    'jest',
    '@typescript-eslint'
  ],

  settings: {
    'import/resolver': 'webpack',
  },
}
