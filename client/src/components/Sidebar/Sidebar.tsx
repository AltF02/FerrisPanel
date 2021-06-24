import React from 'react';

export default function Sidebar() {
  return (
    <aside className="z-30 flex-shrink-0 hidden w-64 overflow-y-auto bg-white dark:bg-gray-800 lg:block">
      <div className="py-4 text-gray-500 dark:text-gray-400">
        <a className="ml-6 text-lg font-bold text-gray-800 dark:text-gray-200" href="/#/app">
          FerrisPanel
        </a>
      </div>
    </aside>
  );
}
