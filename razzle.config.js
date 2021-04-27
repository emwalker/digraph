'use strict';

const path = require('path');

module.exports = {
  modifyWebpackConfig(opts) {
    const config = Object.assign({}, opts.webpackConfig);

    config.resolve.alias = {
      components: path.resolve(__dirname, './src/components'),
      mutations: path.resolve(__dirname, './src/mutations'),
      utils: path.resolve(__dirname, './src/utils'),
    };

    return config;
  },

  plugins: [
    'scss',
    {
      name: 'typescript',
      options: {
        useBabel: true,
      },
    }
  ],
};
