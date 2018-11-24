var path = require('path')
var webpack = require('webpack')
var autoprefixer = require('autoprefixer')
var precss = require('precss')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')

var prodMode = process.env.NODE_ENV === 'production'

const plugins = [
  new webpack.NoEmitOnErrorsPlugin(),
  new MiniCssExtractPlugin({
    filename: prodMode ? '[name].[hash].css' : '[name].css',
    chunkFilename: prodMode ? '[id].[hash].css' : '[id].css',
  }),
  new webpack.ProvidePlugin({
    $: "jquery",
    jQuery: "jquery"
  }),
  new webpack.HotModuleReplacementPlugin({
    multiStep: true
  })
]

if (prodMode) {
  plugins = plugins.concat([
    new webpack.optimize.UglifyJsPlugin({
      output: {comments: false},
      test: /bundle\.js?$/
    }),
    new webpack.DefinePlugin({
      'process.env': {NODE_ENV: JSON.stringify('production')}
    }),
  ])
}

const modules = {
  rules: [
    {test: /\.(png|gif)$/, loader: 'url-loader?name=[name]@[hash].[ext]&limit=5000'},
    {test: /\.svg$/, loader: 'url-loader?name=[name]@[hash].[ext]&limit=5000!svgo-loader?useConfig=svgo1'},
    {test: /\.(pdf|ico|jpg|eot|otf|woff|ttf|mp4|webm)$/, loader: 'file-loader?name=[name]@[hash].[ext]'},
    {test: /\.json$/, loader: 'json-loader'},
    {
      test: /\.jsx?$/,
      include: path.join(__dirname, 'src'),
      loaders: ['babel-loader']
    },
    {
      test: /\.(sa|sc|c)ss$/,
      use: [
        prodMode ? MiniCssExtractPlugin.loader : 'style-loader',
        'css-loader',
        'postcss-loader',
        'sass-loader',
      ],
    },
    {
      test: /\.mjs$/,
      include: /node_modules/,
      type: "javascript/auto",
    },
  ]
}

const config  = {
  devServer: {
    historyApiFallback: true,
    hot: true,
    inline: true,

    host: 'localhost',
    port: 3001,
    proxy: {
      '^/api/*': {
        target: 'http://localhost:8080/api/',
        secure: false
      }
    }
  },

  entry: [
    'babel-polyfill',
    path.join(__dirname, 'src/client'),
  ],

  externals: [
    {
      'isomorphic-fetch': {
        root: 'isomorphic-fetch',
        commonjs2: 'isomorphic-fetch',
        commonjs: 'isomorphic-fetch',
        amd: 'isomorphic-fetch'
      }
    }
  ],

  mode: 'development',

  module: modules,

  node: {
    fs: 'empty',
    module: 'empty'
  },

  output: {
    path: path.join(__dirname, 'src/static/build'),
    publicPath: '/',
    filename: 'bundle.js'
  },

  plugins: plugins,

  resolve: {
    extensions: ['.mjs', '.js', '.jsx', '.css', '.scss'],
    alias: {
      components: path.resolve('src/components'),
      mutations: path.resolve('src/mutations'),
      utils: path.resolve('src/utils'),
    },
  }
}

module.exports = config
