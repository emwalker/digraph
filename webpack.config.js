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
}

module.exports = config
