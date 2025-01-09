import React from 'react';
import { createRoot } from 'react-dom/client';
import Router from './router';

const rootEl = document.getElementById('root');
if (rootEl) {
  const root = createRoot(rootEl);
  root.render(
    <React.StrictMode>
      <Router />
    </React.StrictMode>,
  );
}
