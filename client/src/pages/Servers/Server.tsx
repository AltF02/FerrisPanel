import React from 'react';
import { useParams } from 'react-router';

interface Params {
  serverId: string
}

export default function Server() {
  const { serverId } = useParams<Params>();

  return (
    <div>
      {serverId}
    </div>
  );
}
