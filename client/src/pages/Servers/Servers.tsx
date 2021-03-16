import React from 'react';
import PageTitle from '../../components/Typography/PageTitle';
import ServersList from '../../components/ServersList';

export default function Servers() {
  return (
    <div>
      <PageTitle>Servers</PageTitle>

      <ServersList />
    </div>
  );
}
