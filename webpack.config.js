const path = require('path');

module.exports = {
  watch: true,
  entry: './front/src/index.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'front/dist'),
  },
  module: {
  	rules: [
        {
          test: /\.(?:js|mjs|cjs)$/,
          exclude: /node_modules/,
          use: {
            loader: 'babel-loader',
            options: {
              presets: [
              	["@babel/preset-react", {"runtime": "automatic"}],
                ['@babel/preset-env', { targets: "defaults" }]
              ]
            }
          }
        }
      ]
  }
};