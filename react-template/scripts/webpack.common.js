const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const AutoRoutePlugin = require('@elzajs/auto-route-plugin');
const WebpackBar = require('webpackbar');

const isDev = process.env.NODE_ENV === 'development';

const getStyleLoader = (openCssModule = false) => {
  const loader = ['style-loader', 'css-loader', 'postcss-loader', 'sass-loader'];
  if (openCssModule) {
    loader[1] = {
      loader: 'css-loader',
      options: {
        modules: {
          localIdentName: '[name]__[local]-[hash:base64:5]',
        },
      },
    };
  }
  return loader;
};

const filename = isDev ? '[name].js' : 'static/js/[name].[contenthash:8].js';

module.exports = {
  entry: path.resolve(__dirname, '../src/index.jsx'),
  output: {
    filename,
    clean: true,
    path: path.resolve(__dirname, '../dist'),
  },
  resolve: {
    extensions: ['.js', '.jsx'],
    alias: {
      '@': path.resolve(__dirname, '../src'),
    },
  },
  cache: {
    type: 'filesystem',
    buildDependencies: {
      config: [
        path.resolve(__dirname, 'webpack.common.js'),
        path.resolve(__dirname, '../package.json'),
      ],
    },
    cacheDirectory: path.resolve(__dirname, '../node_modules/.webpack'),
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
        },
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader', 'postcss-loader'],
      },
      {
        test: /\.scss$/,
        oneOf: [
          {
            resourceQuery: /css_modules/,
            use: getStyleLoader(true),
          },
          {
            use: getStyleLoader(false),
          },
        ],
      },
      {
        test: /\.(png|jpe?g|gif|webp|svg|bmp)$/,
        type: 'asset',
        parser: {
          dataUrlCondition: {
            maxSize: 10 * 1024, // 小于10kb的图片会被base64处理
          },
        },
        generator: {
          // 将图片文件输出到 static/images 目录中
          filename: 'static/images/[name].[hash:8].[ext]',
        },
      },
      {
        test: /\.(woff2?|eot|ttf|otf)$/,
        type: 'asset/resource',
        generator: {
          // 将字体文件输出到 static/fonts 目录中
          filename: 'static/fonts/[name].[hash:8].[ext]',
        },
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, '../public/index.html'),
    }),
    new AutoRoutePlugin({ routingMode: 'hash', onlyRoutes: false, indexPath: '/home' }),
    new CopyPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, '../public'),
          to: path.resolve(__dirname, '../dist'),
          globOptions: {
            ignore: ['**/index.html'],
          },
          noErrorOnMissing: true, // 设置为true，即使目标文件夹不存在也不报错
        },
      ],
    }),
    new WebpackBar({
      name: 'webpack',
      color: '#41b883',
    }),
    !isDev &&
      new MiniCssExtractPlugin({
        filename: 'static/css/[name].[contenthash:8].css',
        chunkFilename: 'static/css/[name].[contenthash:8].chunk.css',
      }),
  ].filter(Boolean),
};
