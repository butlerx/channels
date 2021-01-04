module.exports = {
  svelteSortOrder: 'scripts-styles-markup',
  svelteStrictMode: true,
  singleQuote: true,
  trailingComma: 'all',
  proseWrap: 'always',
  printWidth: 100,
  plugins: ['prettier-plugin-svelte'],
  overrides: [
    {
      files: '*.ts',
      options: {
        parser: 'typescript',
      },
    },
  ],
};
