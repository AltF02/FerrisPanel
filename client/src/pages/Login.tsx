import React from 'react';
import { RouteComponentProps } from 'react-router-dom';
import LoginForm from '../components/LoginForm';

interface IProps extends RouteComponentProps {

}

export default function Login(props: IProps) {
  return (
    <div className="flex items-center min-h-screen p-6 bg-gray-50 dark:bg-gray-900">
      <LoginForm {...props} />
    </div>
  );
}
