import React, { useContext, useState } from 'react';
import {
  Badge, Dropdown, DropdownItem, WindmillContext,
} from '@windmill/react-ui';
import {
  MoonIcon,
  SunIcon,
  BellIcon,
  OutlineCogIcon,
  OutlineLogoutIcon,
  UserIcon,
} from '../icons';
import userState from '../state/userState';

function Header() {
  const { mode, toggleMode } = useContext(WindmillContext);

  const [isNotificationsMenuOpen, setIsNotificationsMenuOpen] = useState(false);
  const [isProfileMenuOpen, setIsProfileMenuOpen] = useState(false);
  const { logout } = userState.useContainer();

  function handleNotificationsClick() {
    setIsNotificationsMenuOpen(!isNotificationsMenuOpen);
  }

  function handleProfileClick() {
    setIsProfileMenuOpen(!isProfileMenuOpen);
  }

  function handleLogout() {
    logout();
    window.location.reload();
  }

  return (
    <header className="z-40 py-4 bg-white shadow-bottom dark:bg-gray-800">
      <div className="container flex items-center justify-between h-full px-6 mx-auto text-purple-600 dark:text-purple-300">
        <div />
        <ul className="flex items-center flex-shrink-0 space-x-6">

          <li className="flex">
            <button
              type="button"
              className="rounded-md focus:outline-none focus:shadow-outline-purple"
              onClick={toggleMode}
              aria-label="Toggle color mode"
            >
              {mode === 'dark' ? (
                <SunIcon className="w-5 h-5" aria-hidden="true" />
              ) : (
                <MoonIcon className="w-5 h-5" aria-hidden="true" />
              )}
            </button>
          </li>
          <li className="relative">
            <button
              type="button"
              className="relative align-middle rounded-md focus:outline-none focus:shadow-outline-purple"
              onClick={handleNotificationsClick}
              aria-label="Notifications"
              aria-haspopup="true"
            >
              <BellIcon className="w-5 h-5" aria-hidden="true" />
              <span
                aria-hidden="true"
                className="absolute top-0 right-0 inline-block w-3 h-3 transform translate-x-1 -translate-y-1 bg-red-600 border-2 border-white rounded-full dark:border-gray-800"
              />
            </button>

            <Dropdown
              align="right"
              isOpen={isNotificationsMenuOpen}
              onClose={() => setIsNotificationsMenuOpen(false)}
            >
              <DropdownItem tag="a" href="#" className="justify-between">
                <span>Messages</span>
                <Badge type="danger">4</Badge>
              </DropdownItem>
              <DropdownItem>
                <span>Alerts</span>
              </DropdownItem>
            </Dropdown>
          </li>

          <li className="relative">
            <button
              type="button"
              className="relative align-middle rounded-md focus:outline-none focus:shadow-outline-purple"
              onClick={handleProfileClick}
              aria-label="Account"
              aria-haspopup="true"
            >
              <UserIcon className="w-5 h-5" aria-hidden="true" />
            </button>
            <Dropdown
              align="right"
              isOpen={isProfileMenuOpen}
              onClose={() => setIsProfileMenuOpen(false)}
            >
              <DropdownItem tag="a" href="#">
                <OutlineCogIcon className="w-4 h-4 mr-3" aria-hidden="true" />
                <span>Settings</span>
              </DropdownItem>
              <DropdownItem onClick={handleLogout}>
                <OutlineLogoutIcon className="w-4 h-4 mr-3" aria-hidden="true" />
                <span>Log out</span>
              </DropdownItem>
            </Dropdown>
          </li>
        </ul>
      </div>
    </header>
  );
}

export default Header;
