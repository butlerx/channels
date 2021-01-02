module.exports = {
  env: {
    es6: true,
    browser: true,
  },
  parser: 'babel-eslint',
  parserOptions: {
    ecmaVersion: 2019,
    sourceType: 'module',
  },
  root: true,
  extends: [
    'airbnb',
    'plugin:@typescript-eslint/recommended',
    'prettier',
    'prettier/@typescript-eslint',
  ],
  plugins: ['svelte3', '@typescript-eslint', 'prettier'],
  overrides: [
    {
      files: ['**/*.svelte'],
      processor: 'svelte3/svelte3',
    },
  ],
  rules: {
    'arrow-parens': ['error', 'as-needed'],
    'func-style': ['error', 'declaration', { allowArrowFunctions: true }],
    'import/prefer-default-export': 0,
    'linebreak-style': ['error', 'unix'],
    'no-param-reassign': ['error', { props: false }],
    'no-use-before-define': ['error', { functions: false }],
  },
};
