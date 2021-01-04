/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  plugins: ['@snowpack/plugin-svelte'],
  mount: {
    public: '/',
    src: '/_dist_',
  },
  proxy: {
    '/api': {
      target: 'http://localhost:3030',
      ws: true,
    },
  },
};
