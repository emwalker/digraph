/* eslint import/no-unresolved: 0 */
require('normalize.css')
require('./global')

/**
 * Components.
 * Include all css files just if you need
 * to hot reload it. And make sure that you
 * use `webpack.optimize.DedupePlugin`
 */
require('components/app/styles')
require('components/homepage/styles')
