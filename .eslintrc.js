module.exports = {
  "extends": "airbnb",

  "rules": {
    "curly": ["error", "multi-or-nest", "consistent"],
    "semi": ["error", "never"],
    // Needed for semi: ["error", never"]
    "no-unexpected-multiline": "error",
    "flowtype/define-flow-type": 1,
    "flowtype/use-flow-type": 1,
    "jsx-a11y/anchor-is-valid": ["error", "never"],
    "function-paren-newline": ["error", "consistent"],
    "object-curly-newline": ["error",  {"consistent": true}],
  },

  "parser": "babel-eslint",

  "env": {
    "browser": true,
    "jest": true,
  },

  "plugins": [
    "react",
    "jest",
    "flowtype",
  ],

  "settings": {
    "import/resolver": "webpack"
  },
};

