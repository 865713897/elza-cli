import React from 'react';
import { createRoot } from 'react-dom/client';
import Home from './pages/Home';

const rootEl = document.getElementById('root');
if (rootEl) {
  const root = createRoot(rootEl);
  root.render(
    <React.StrictMode>
      <Home />
    </React.StrictMode>
  );
}
