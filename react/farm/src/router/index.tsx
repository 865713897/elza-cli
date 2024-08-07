import React, { useEffect, useState } from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import { getRoutes } from './routes';

interface IRoute {
  path: string;
  name: string;
  Component: React.FC;
  children?: IRoute[];
}

export default function AppRouter() {
  const [routes, setRoutes] = useState<IRoute[]>([]);

  useEffect(() => {
    setRoutes(getRoutes());
  }, []);

  const renderRoutes = (routes: IRoute[]) => {
    return routes.map((route) => {
      const { path, Component, children = [] } = route || {};
      return (
        <Route key={path} path={path} element={<Component />}>
          {renderRoutes(children)}
        </Route>
      );
    })
  }

  if (!routes.length) {
    return <div>Loading...</div>;
  }

  return (
    <Router>
      <Routes>
        {renderRoutes(routes)}
        <Route path="*" element={<Navigate to="/home" />} />
      </Routes>
    </Router>
  );
}
  