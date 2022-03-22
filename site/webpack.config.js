const path = require('path');
// module.exports = {
//   entry: "./src/test.ts",
//   output: {
//     path: path.resolve(__dirname, "webpack_dist"),
//     filename: "index.js",
//   },
//   mode: "development"
// };

// const path = require('path');

// https://webpack.js.org/guides/typescript/
module.exports = {
  entry: './src/test.ts',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
    // buildHttp: true,
    // layers: true,
    // lazyCompilation: true,
    // outputModule: true,
    syncWebAssembly: true,
    topLevelAwait: true,
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'webpack_dist'),
  },
  mode: "production"
};