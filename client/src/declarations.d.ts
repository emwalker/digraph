declare module '*.css';
declare module '*.scss';
declare module 'react-async-ssr';
declare module 'found-relay';
declare module 'draft-js-single-line-plugin';
declare module 'es6-promise-debounce';

declare module 'babel-plugin-relay/macro' {
  export { graphql as default } from 'react-relay';
}
