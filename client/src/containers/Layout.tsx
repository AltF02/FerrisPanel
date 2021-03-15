import React, { Suspense } from 'react';
import { Route } from 'react-router-dom';
import routes from '../routes';
import Main from './Main';
import Header from '../components/Header';
import Sidebar from '../components/Sidebar/Sidebar';

// const Page404 = lazy(() => import('../pages/404'));

export default function Layout() {
  return (
    <div className="flex h-screen bg-gray-50 dark:bg-gray-900">
      <Sidebar />

      <div className="flex flex-col flex-1 w-full">
        <Header />

        <Main>
          <Suspense fallback={<div>Loading...</div>}>
            {
              routes.map((route) => (route.component ? (
                // @ts-ignore
                <Route
                  key={route.id}
                  exact
                  path={`/app${route.path}`}
                  render={(props) =>
                    // @ts-ignore
                    // eslint-disable-next-line implicit-arrow-linebreak
                    <route.component {...props} />}
                />
              ) : null))
            }
          </Suspense>
        </Main>
      </div>
    </div>
  );
}
