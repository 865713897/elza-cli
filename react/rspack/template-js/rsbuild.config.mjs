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
        new AutoRoutesPlugin({dir: './src/pages', moduleType: 'jsx'}),
      ],
    },
  },
  plugins: [pluginReact(), `placeholder:1`],
});
