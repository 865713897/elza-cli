import path from 'path';
import { Configuration, DefinePlugin } from 'webpack';
import { Configuration as DevServerConfiguration } from 'webpack-dev-server';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import CopyPlugin from 'copy-webpack-plugin';
import MiniCssExtractPlugin from 'mini-css-extract-plugin';
import WebpackBar from 'webpackbar';
import AutoRoutePlugin from 'webpack-plugin-auto-routes';

interface WebpackDevServerConfiguration {
  devServer?: DevServerConfiguration;
}

type WebpackConfiguration = Configuration & WebpackDevServerConfiguration;

const isDev: boolean = process.env.NODE_ENV === 'development';

const getStyleLoader = (openCssModule = false) => {
  const loader: any = [
    isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
    'css-loader',
    'postcss-loader',
    'sass-loader',
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

const filename: string = isDev
  ? '[name].js'
  : 'static/js/[name].[chunkhash:8].js';

const baseConfig: WebpackConfiguration = {
  entry: path.resolve(__dirname, '../src/index.tsx'),
  output: {
    path: path.resolve(__dirname, '../dist'),
    filename,
    clean: true,
    publicPath: '/',
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, '../src'),
    },
    extensions: ['.ts', '.tsx', '.js', '.jsx'],
  },
  cache: {
    type: 'filesystem',
    buildDependencies: {
      config: [
        path.resolve(__dirname, 'webpack.common.ts'),
        path.resolve(__dirname, '../package.json'),
      ],
    },
    cacheDirectory: path.resolve(__dirname, '../node_modules/.webpack'),
  },
  module: {
    rules: [
      {
        test: /\.(j|t)sx?$/,
        use: 'babel-loader',
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
        exclude: /node_modules/,
      },
      {
        test: /\.(png|jpe?g|gif|webp|svg)$/,
        type: 'asset',
        parser: {
          dataUrlCondition: {
            maxSize: 10 * 1024, // 小于10kb的图片会被base64处理
          },
        },
        generator: {
          filename: 'static/image/[name].[contenthash:8][ext]',
        },
      },
      {
        test: /\.(woff2?|eot|ttf|otf)$/,
        type: 'asset/resource',
        generator: {
          filename: 'static/fonts/[name].[contenthash:8][ext]',
        },
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
      inject: true,
    }),
    new AutoRoutePlugin({
      routingMode: 'hash',
      onlyRoutes: false,
      indexPath: '/home',
    }),
    new DefinePlugin({
      'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV),
    }),
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

export default baseConfig;
