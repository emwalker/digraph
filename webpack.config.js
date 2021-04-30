const path = require('path')

const config = {
  resolve: {
    extensions: ['.mjs', '.js', '.jsx', '.ts', '.tsx', '.css', '.scss'],

    alias: {
      components: path.resolve('src/components'),
      mutations: path.resolve('src/mutations'),
      utils: path.resolve('src/utils'),
    },

  },

  module: {
    rules: [
      {
        test: /\.(gif|jpe?g|png|ico)$/,
        loader: 'url-loader?limit=10000'
      }
    ],
  }
}

module.exports = config
