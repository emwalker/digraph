'use strict';

const path = require('path');

module.exports = {
  modify(config, { target, dev }, webpack) {
    const appConfig = Object.assign({}, config);

    appConfig.resolve.alias = {
      components: path.resolve(__dirname, './src/components'),
      mutations: path.resolve(__dirname, './src/mutations'),
      utils: path.resolve(__dirname, './src/utils'),
    };

    return appConfig;
  },

  plugins: ['scss']
};
