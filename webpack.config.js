const path = require('path')
const webpack = require('webpack')
const autoprefixer = require('autoprefixer')
const precss = require('precss')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const ManifestPlugin = require('webpack-manifest-plugin')

const prodMode = process.env.NODE_ENV === 'production'

var plugins = [
  new ManifestPlugin({
    writeToFileEmit: true,
    publicPath: '/static/',
  }),
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
  }),
  new webpack.SourceMapDevToolPlugin({
    filename: '[name].js.map',
    exclude: ['vendor.js'],
  }),
]

if (prodMode) {
  plugins = plugins.concat([
    new webpack.DefinePlugin({
      'process.env': {NODE_ENV: JSON.stringify('production')}
    }),
    new webpack.optimize.AggressiveMergingPlugin(),
  ])
}

const modules = {
  rules: [
    {test: /\.(png|gif)$/, loader: 'url-loader?name=[name]@[hash].[ext]&limit=5000'},
    {test: /\.svg$/, loader: 'url-loader?name=[name]@[hash].[ext]&limit=5000!svgo-loader?useConfig=svgo1'},
    {test: /\.(pdf|ico|jpg|eot|otf|woff|ttf|mp4|webm)$/, loader: 'file-loader?name=[name]@[hash].[ext]'},
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
      type: 'javascript/auto',
    },
  ],
}

const config  = {
  devtool: false,

  devServer: {
    historyApiFallback: true,
    hot: true,
    inline: true,
    port: 3001,
    host: 'localhost',
    proxy: {
      '/graphql': {
        target: 'http://localhost:8080/',
        secure: false,
      },
      '/logout': {
        target: 'http://localhost:8080/',
        secure: false,
      },
      '/auth': {
        target: 'http://localhost:8080/',
        secure: false,
      },
    },
  },

  entry: {
    main: [
      path.join(__dirname, 'src/client.jsx'),
    ],
  },

  externals: [
    {
      'isomorphic-fetch': {
        root: 'isomorphic-fetch',
        commonjs2: 'isomorphic-fetch',
        commonjs: 'isomorphic-fetch',
        amd: 'isomorphic-fetch'
      },
    },
  ],

  mode: process.env.NODE_ENV,

  module: modules,

  node: {
    fs: 'empty',
    module: 'empty'
  },

  output: {
    path: path.join(__dirname, 'public/webpack'),
    publicPath: '/',
    filename: prodMode ? '[name].[hash].js' : 'bundle.js',
  },

  plugins: plugins,

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
