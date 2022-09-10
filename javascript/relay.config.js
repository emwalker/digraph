module.exports = {
  src: "./src",
  language: "typescript",
  schema: "../schema.graphql",
  artifactDirectory: './src/__generated__',
  exclude: ["**/node_modules/**", "**/__mocks__/**", "**/__generated__/**"],
  customScalars: {
    color: 'string',
  }
};
