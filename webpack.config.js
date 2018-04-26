var path = require('path')
var webpack = require('webpack')
var autoprefixer = require('autoprefixer')
var precss = require('precss')
var functions = require('postcss-functions')
var ExtractTextPlugin = require('extract-text-webpack-plugin')

var postCssLoader = [
  'css-loader?modules',
  '&localIdentName=[name]__[local]___[hash:base64:5]',
  '&disableStructuralMinification',
]

var plugins = [
  new webpack.NoEmitOnErrorsPlugin(),
  new ExtractTextPlugin('bundle.css'),
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
    })
  ])
};

var config  = {
  mode: 'development',

  entry: {
    bundle: path.join(__dirname, 'client/index.js')
  },

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
    path: path.join(__dirname, 'server/data/static/build'),
    publicPath: '/static/build/',
    filename: '[name].js'
  },

  plugins: plugins,

  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          'isomorphic-style-loader',
          {
            loader: 'css-loader',
            options: {
              importLoaders: 1
            }
          }
        ]
      },
      {test: /\.(png|gif)$/, loader: 'url-loader?name=[name]@[hash].[ext]&limit=5000'},
      {test: /\.svg$/, loader: 'url-loader?name=[name]@[hash].[ext]&limit=5000!svgo-loader?useConfig=svgo1'},
      {test: /\.(pdf|ico|jpg|eot|otf|woff|ttf|mp4|webm)$/, loader: 'file-loader?name=[name]@[hash].[ext]'},
      {test: /\.json$/, loader: 'json-loader'},
      {
        test: /\.jsx?$/,
        include: path.join(__dirname, 'client'),
        loaders: ['babel-loader']
      },
      {
        test: /\.scss$/,
        use: [
          {loader: 'style-loader'},
          {loader: 'css-loader'},
          {loader: 'sass-loader'}
        ]
      }
    ]
  },

  resolve: {
    extensions: ['.js', '.jsx', '.css', '.scss'],
    alias: {
      'components': path.join(__dirname, 'client/components')
    }
  }
}

module.exports = config
