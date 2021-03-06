const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const path = require('path')

const config = {
  resolve: {
    extensions: ['.mjs', '.js', '.jsx', '.css', '.scss'],

    alias: {
      components: path.resolve('src/components'),
      mutations: path.resolve('src/mutations'),
      utils: path.resolve('src/utils'),
    },

  },

  plugins: [new MiniCssExtractPlugin()],

  module: {
    rules: [
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, 'css-loader'],
      },
    ],
  }
}

module.exports = config
