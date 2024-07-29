import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
`placeholder:0`
import autoRoutesPlugin from 'webpack-plugin-auto-routes';

export default defineConfig({
  tools: {
    rspack: {
      plugins: [
        new autoRoutesPlugin({
          routingMode: 'browser',
          onlyRoutes: false,
          indexPath: '/home',
        }),
      ],
    },
  },
  plugins: [pluginReact(), `placeholder:1`],
});
