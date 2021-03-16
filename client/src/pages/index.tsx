import {
  Redirect, Route, Switch,
} from 'react-router-dom';
import React, { lazy } from 'react';

const Login = lazy(() => import('./Login'));
const Layout = lazy(() => import('../containers/Layout'));
const Page404 = lazy(() => import('../pages/404'));

export default function Pages() {
  return (
    <Switch>
      <Route path="/login" component={Login} />

      <Redirect exact from="/" to="/login" />

      <Route path="/app" component={Layout} />
      <Route component={Page404} />
    </Switch>
  );
}
