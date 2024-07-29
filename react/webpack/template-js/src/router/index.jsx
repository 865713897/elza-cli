import React, { useEffect, useState } from 'react';
import { HashRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import { getRoutes } from './routes';

export default function AppRouter() {
  const [routes, setRoutes] = useState([]);

  useEffect(() => {
    setRoutes(getRoutes());
  }, []);

  const renderRoutes = (routes) => {
    return routes.map((route) => {
      const { path, Component, children = [] } = route || {};
      return (
        <Route key={path} path={path} element={<Component />}>
          {renderRoutes(children)}
        </Route>
      );
    });
  };

  return (
    <Router>
      <Routes>
        {renderRoutes(routes)}
        <Route path="*" element={<Navigate to="/home" />} />
      </Routes>
    </Router>
  );
}
