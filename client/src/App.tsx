import React from 'react';
import {HashRouter, Link, Switch, Route} from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";


export default class App extends React.Component {
  render() {
      return (
          <HashRouter>
              <div>
                  <Switch>
                      <Route path="/login">
                          <Login />
                      </Route>
                      <Route path="/">
                          <Home />
                      </Route>
                  </Switch>
              </div>
          </HashRouter>
      )
  }
};
