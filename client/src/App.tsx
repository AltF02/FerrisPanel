import React from 'react';
import {
  HashRouter,
} from 'react-router-dom';
import { UserState } from './state';
import Pages from './pages';
import { Cookie } from './providers';

export default function App() {
  return (
    <UserState.Provider>
      <HashRouter>
        <Cookie.Provider>
          <Pages />
        </Cookie.Provider>
      </HashRouter>
    </UserState.Provider>
  );
}
