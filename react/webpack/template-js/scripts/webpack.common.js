const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const AutoRoutesPlugin = require('webpack-plugin-auto-routes');
const BetterInfoPlugin = require('webpack-plugin-better-info');

const isDev = process.env.NODE_ENV === 'development';

const getStyleLoader = (openCssModule = false) => {
  const loader = [
    isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
    'css-loader',
    'postcss-loader',
    '`placeholder:0`',
  ];
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
        test: /\.(j|t)sx?$/,
        use: {
          loader: '`placeholder:2`',
        },
        exclude: /node_modules/,
      },
      {
        test: /\.css$/,
        use: [
          isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
          'css-loader',
          'postcss-loader',
        ],
        exclude: /node_modules/,
      },
      {
        test: /\.`placeholder:1`$/,
        oneOf: [
          {
            resourceQuery: /css_modules/,
            use: getStyleLoader(true),
          },
          {
            use: getStyleLoader(false),
          },
        ],
        exclude: /node_modules/,
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
          filename: 'static/images/[name].[contenthash:8].[ext]',
        },
        exclude: /node_modules/,
      },
      {
        test: /\.(woff2?|eot|ttf|otf)$/,
        type: 'asset/resource',
        generator: {
          filename: 'static/fonts/[name].[contenthash:8].[ext]',
        },
        exclude: /node_modules/,
      },
      {
        test: /.(mp4|webm|ogg|mp3|wav|flac|aac)$/,
        type: 'asset',
        generator: {
          filename: 'static/media/[name].[contenthash:8][ext]',
        },
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, '../public/index.html'),
    }),
    new AutoRoutesPlugin({dir: './src/pages', moduleType: 'jsx'}),
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
    new BetterInfoPlugin({}),
    !isDev &&
      new MiniCssExtractPlugin({
        filename: 'static/css/[name].[contenthash:8].css',
        chunkFilename: 'static/css/[name].[contenthash:8].chunk.css',
      }),
  ].filter(Boolean),
};
