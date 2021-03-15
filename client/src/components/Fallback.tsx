import React from 'react';

export default function Fallback() {
  return (
    <div className="flex flex-col items-center w-full h-screen p-6 text-lg dark:bg-gray-900">
      <h1 className="font-medium text-gray-600 dark:text-gray-400">
        Loading...
      </h1>
    </div>
  );
}
