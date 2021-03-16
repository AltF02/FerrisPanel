import React from 'react';

export default function PageTitle({ children }: { children: React.ReactNode }) {
  return <h1 className="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">{children}</h1>;
}
