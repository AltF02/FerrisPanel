import {
  Redirect, Route, Switch, useHistory, useLocation,
} from 'react-router-dom';
import React, { lazy, useEffect } from 'react';
import { UserState } from '../state';

const Login = lazy(() => import('./Login'));
const Layout = lazy(() => import('../containers/Layout'));

export default function Pages() {
  const { authenticated } = UserState.useContainer();
  const location = useLocation();
  const history = useHistory();

  useEffect(() => {
    if (!authenticated && !location.pathname.startsWith('/login')) {
      history.push('/login');
    }
  }, [location.pathname]);

  return (
    <Switch>
      <Route path="/login" component={Login} />

      <Redirect exact from="/" to="/login" />

      <Route path="/app" component={Layout} />
    </Switch>
  );
}
