import { defineConfig } from '@farmfe/core';`placeholder:0`;
import farmAutoRoutes from 'farm-plugin-auto-routes';

export default defineConfig({
  plugins: [
    '@farmfe/plugin-react',
    `placeholder:1`,
    farmAutoRoutes({ writeToDisk: true }),
  ],
});
