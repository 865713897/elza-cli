// @ts-nocheck
import React, { Suspense } from 'react';

function withLazyLoad<P>(LazyComponent: React.ComponentType<P>) {
  const lazyComponentWrapper: React.FC<P> = (props) => (
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
      Component: withLazyLoad(React.lazy(() => import(/* webpackChunkName: "src__pages__home__index" */ '../pages/home/index.tsx'))),
      children: []
    }
  ];
  return routes;
}
