import React from 'react';
import { createRoot } from 'react-dom/client';
import Router from './router';

const root = createRoot(document.getElementById('root') as HTMLElement);

function renderApp() {
  root.render(<Router />);
}

renderApp();

if (module.hot) {
  module.hot.accept('./router', renderApp);
}
