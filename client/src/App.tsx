import React from 'react';
import {
  HashRouter,
} from 'react-router-dom';
import { UserState } from './state';
import Pages from './pages';

export default function App() {
  return (
    <UserState.Provider>
      <HashRouter>
        <Pages />
      </HashRouter>
    </UserState.Provider>
  );
}
