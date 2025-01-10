import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';
import AutoRoutesPlugin from 'farm-plugin-auto-routes';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), AutoRoutesPlugin({ writeToDisk: true })],
});
