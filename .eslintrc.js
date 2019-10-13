module.exports = {
  "extends": "airbnb",

  "rules": {
    "curly": ["error", "multi-or-nest", "consistent"],
    "flowtype/define-flow-type": 1,
    "flowtype/use-flow-type": 1,
    "flowtype/require-valid-file-annotation": [
      2,
      "always"
    ],
    "jsx-a11y/anchor-is-valid": "off",
    "jsx-a11y/label-has-associated-control": "off",
    "jsx-a11y/label-has-for": "off",
    // Needed for semi: ["error", never"]
    "no-unexpected-multiline": "error",
    "object-curly-newline": ["error",  {"consistent": true}],
    "react/destructuring-assignment": "off",
    "react/jsx-props-no-spreading": "off",
    "react/static-property-placement": ["error", "static public field"],
    "semi": ["error", "never"]
  },

  "overrides": [
    {
      "files": [ "**/*.test.jsx" ],
      "rules": {
        "flowtype/require-valid-file-annotation": [
          2,
          "never"
        ]
      }
    }
  ],

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
