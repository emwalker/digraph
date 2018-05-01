var path = require('path')
var webpack = require('webpack')
var autoprefixer = require('autoprefixer')
var precss = require('precss')
var functions = require('postcss-functions')
var ExtractTextPlugin = require('extract-text-webpack-plugin')

var plugins = [
  new webpack.NoEmitOnErrorsPlugin(),
  new ExtractTextPlugin({filename: 'style.css', allChunks: true}),
  new webpack.ProvidePlugin({
    $: "jquery",
    jQuery: "jquery"
  }),
]

if (process.env.NODE_ENV === 'production') {
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

const scssLoader = [
  { loader: 'css-loader' },
  { loader: 'sass-loader' },
]

var config  = {
  mode: 'development',

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

  output: {
    path: path.join(__dirname, 'src/static/build'),
    publicPath: '/',
    filename: 'bundle.js'
  },

  plugins: plugins,

  module: {
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
        loader: ExtractTextPlugin.extract({
          use: 'css-loader',
          fallback: 'style-loader',
        }),
        test: /\.css$/,
      },
      {
        test: /\.(scss|sass)$/,
        loader: ExtractTextPlugin.extract({
          use: ['css-loader', 'sass-loader'],
          fallback: 'style-loader',
        })
      },
    ]
  },

  resolve: {
    extensions: ['.js', '.jsx', '.css', '.scss'],
    alias: {
      'components': path.join(__dirname, 'src/components')
    }
  }
}

module.exports = config
