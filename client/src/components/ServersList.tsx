import React from 'react';
import ServerCard from './cards/ServerCard';

interface IProps {
  limit?: number
}

// eslint-disable-next-line no-unused-vars
export default function ServersList({ limit }: IProps) {
  return (
    <div className="mb-6">
      <ServerCard id="test" description="Test!" name="Test Server" />
      <ServerCard id="test" description="Test!" name="Test Server" />
      <ServerCard id="test" description="Test!" name="Test Server" />
      <ServerCard id="test" description="Test!" name="Test Server" />
      <ServerCard id="test" description="Test!" name="Test Server" />
    </div>
  );
}

ServersList.defaultProps = {
  limit: 0,
};
