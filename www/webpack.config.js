const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      { from: 'static' }
    ]),
  ],
  module: {
    rules: [
          {
            test: /\.glsl$/i,
            use: 'raw-loader',
          },
          {
            test:/\.css$/,
            use:['style-loader','css-loader']
          }
    ],
  },
  resolve: {
    alias: {
      'vue$': 'vue/dist/vue.esm.js' ,
    }
  }
};
