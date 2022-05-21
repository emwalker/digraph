const path = require('path')
const CopyWebpackPlugin = require('copy-webpack-plugin');

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

    modules: [
      'node_modules',
      path.resolve(__dirname, 'public'),
    ]
  },

  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: 'javascript/public' }
      ],
    }),
  ],
}

module.exports = config
