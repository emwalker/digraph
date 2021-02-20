'use strict';

const path = require('path');

module.exports = {
  modifyWebpackConfig(opts) {
    const appConfig = Object.assign({}, opts.webpackConfig);

    appConfig.resolve.alias = {
      components: path.resolve(__dirname, './src/components'),
      mutations: path.resolve(__dirname, './src/mutations'),
      utils: path.resolve(__dirname, './src/utils'),
    };

    return appConfig;
  },

  plugins: ['scss']
};
