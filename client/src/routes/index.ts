import { lazy } from 'react';

const Dashboard = lazy(() => import('../pages/Dashboard'));
const Servers = lazy(() => import('../pages/Servers/Servers'));
const Server = lazy(() => import('../pages/Servers/Server'));
const Settings = lazy(() => import('../pages/Settings'));

const routes = [
  {
    path: '/',
    component: Dashboard,
    id: 1,
  },
  {
    path: '/servers',
    component: Servers,
    id: 2,
  },
  {
    path: '/server/:serverId',
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
