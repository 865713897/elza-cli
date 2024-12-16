import React from 'react';
import { HashRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import { getRoutes } from './routes';

interface RouteConfig {
  id: string;
  parentId?: string;
  path: string;
  isLayout?: boolean;
  [key: string]: any;
}

type RoutesMap = Record<string, RouteConfig>;
type RouteComponentsMap = Record<string, React.ComponentType<any>>;

export default function AppRouter() {
  const { routes, routeComponents }: { routes: RoutesMap; routeComponents: RouteComponentsMap } = getRoutes();

  const renderRoutes = () => {
    return Object.keys(routeComponents).map((key) => {
      const { id, parentId, path, isLayout } = routes[key];
      if (isLayout) return null;
      const LayoutComponent = parentId ? routeComponents[parentId] : null;
      const Component = routeComponents[id];
      if (LayoutComponent) {
        return (
          <Route element={<LayoutComponent />} key={key}>
            <Route key={id} path={path} element={<Component />} />
          </Route>
        );
      }
      return <Route key={id} path={path} element={<Component />} />;
    });
  };

  return (
    <Router>
      <Routes>
        {renderRoutes()}
        <Route path="*" element={<Navigate to="/home" />} />
      </Routes>
    </Router>
  );
}
