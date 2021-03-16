import { lazy } from 'react';

const Home = lazy(() => import('../pages/Home'));
const Servers = lazy(() => import('../pages/Servers/Servers'));
const Server = lazy(() => import('../pages/Servers/Server'));
const Settings = lazy(() => import('../pages/Settings'));

const routes = [
  {
    path: '/',
    component: Home,
    id: 1,
  },
  {
    path: '/servers',
    component: Servers,
    id: 2,
  },
  {
    path: '/servers/:serverId',
    component: Server,
    id: 3,
  },
  {
    path: '/settings',
    component: Settings,
    id: 4,
  },
];

export default routes;
