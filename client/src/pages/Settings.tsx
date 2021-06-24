import React from 'react';
import { Button, Input, Label } from '@windmill/react-ui';
import PageTitle from '../components/Typography/PageTitle';
import SectionTitle from '../components/Typography/SectionTitle';
import { UserState } from '../state';

export default function Settings() {
  const { userData } = UserState.useContainer();

  return (
    <div>
      <PageTitle>Settings</PageTitle>

      <SectionTitle>Update Password</SectionTitle>

      <div className="px-4 py-3 mb-8 bg-white rounded-lg dark:bg-gray-800">
        <Label>
          <span>Current Password</span>
          <Input className="mt-1" placeholder="•••••••••" type="password" css="" />
        </Label>

        <Label className="mt-4">
          <span>New Password</span>
          <Input className="mt-1" placeholder="•••••••••" type="password" css="" />
        </Label>

        <Label className="mt-4">
          <span>Confirm New Password</span>
          <Input className="mt-1" placeholder="•••••••••" type="password" css="" />
        </Label>

        <Button className="mt-4 w-full">
          Change Password
        </Button>
      </div>

      <SectionTitle>Update Email</SectionTitle>

      <div className="px-4 py-3 mb-8 bg-white rounded-lg dark:bg-gray-800">
        <Label>
          <span>New Email</span>
          <Input className="mt-1" defaultValue={(userData || { email: 'foo@bar.com' }).email} css="" />
        </Label>

        <Label className="mt-4">
          <span>Password</span>
          <Input className="mt-1" placeholder="•••••••••" type="password" css="" />
        </Label>

        <Button className="mt-4 w-full">
          Change Email
        </Button>
      </div>
    </div>
  );
}
