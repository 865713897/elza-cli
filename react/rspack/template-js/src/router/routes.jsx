import React, { Suspense } from 'react';
    
function withLazyLoad(LazyComponent) {
  const lazyComponentWrapper = (props) => (
    <Suspense fallback={<div>Loading...</div>}>
      <LazyComponent {...props} />
    </Suspense>
  );

  return lazyComponentWrapper;
}

export function getRoutes() {
  const routes = [
    {
      path: '/home',
      name: 'home',
      Component: withLazyLoad(React.lazy(() => import(/* webpackChunkName: "src__pages__home__index" */ '../pages/home/index.jsx'))),
      children: []
    }
  ];
  return routes;
}