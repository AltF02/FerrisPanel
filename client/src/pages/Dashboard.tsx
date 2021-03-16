import React from 'react';
import PageTitle from '../components/Typography/PageTitle';
import SectionTitle from '../components/Typography/SectionTitle';
import ServersList from '../components/ServersList';

export default function Dashboard() {
  return (
    <div>
      <PageTitle>Dashboard</PageTitle>

      <SectionTitle>Servers</SectionTitle>
      <ServersList limit={5} />

      <SectionTitle>Logs</SectionTitle>
    </div>
  );
}
