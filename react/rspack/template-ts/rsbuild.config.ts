import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';`placeholder:0`
import AutoRoutesPlugin from 'webpack-plugin-auto-routes';

export default defineConfig({
  html: {
    template: './public/index.html',
  },
  tools: {
    rspack: {
      plugins: [
        new AutoRoutesPlugin({
          routingMode: 'browser',
          onlyRoutes: false,
          indexPath: '/home',
        }),
      ],
    },
  },
  plugins: [pluginReact(), `placeholder:1`],
});
