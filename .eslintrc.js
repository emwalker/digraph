module.exports = {
  "extends": "airbnb",

  "rules": {
    "curly": ["error", "multi-or-nest", "consistent"],
    "flowtype/define-flow-type": 1,
    "flowtype/use-flow-type": 1,
    "function-paren-newline": ["error", "consistent"],
    "jsx-a11y/anchor-is-valid": ["error", "never"],
    "jsx-a11y/label-has-associated-control": "off",
    "jsx-a11y/label-has-for": "off",
    // Needed for semi: ["error", never"]
    "no-unexpected-multiline": "error",
    "object-curly-newline": ["error",  {"consistent": true}],
    "react/destructuring-assignment": "off",
    "semi": ["error", "never"]
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

