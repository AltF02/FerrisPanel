import React, { useEffect, useState } from 'react';
import {
  Button, Input, Label,
} from '@windmill/react-ui';
import { Link, RouteComponentProps } from 'react-router-dom';
import useFormInput from '../utils/form';
import { UserState } from '../state';

interface IProps extends RouteComponentProps {
}

export default function Login(props: IProps) {
  const email = useFormInput('');
  const password = useFormInput('');

  const [error, setError] = useState(false);
  const [remember, setRemember] = useState(false);

  const { authenticate, loading, authenticated } = UserState.useContainer();

  useEffect(() => {
    if (authenticated) {
      props.history.push('app');
    }
  });

  const handleLogin = () => {
    authenticate(email.value, password.value, remember).then((err) => {
      setError(err);
      if (!err) {
        props.history.push('app');
      }
    }).catch(setError);
  };

  return (
    <div className="flex items-center min-h-screen p-6 bg-gray-50 dark:bg-gray-900">
      <div className="flex-1 h-full max-w-lg mx-auto overflow-hidden bg-white rounded-lg shadow-xl dark:bg-gray-800">
        <div className="flex flex-col overflow-y-auto md:flex-row">
          <form className="flex items-center justify-center p-6 sm:p-12 w-full">
            <div className="w-full">
              <h1 className="mb-4 text-xl font-semibold text-gray-700 dark:text-gray-200">Login to FerrisPanel</h1>
              <Label className="mt-4">
                <span>Email</span>
                <Input
                  valid={error ? false : undefined}
                  className="mt-1"
                  type="email"
                  placeholder="foo@bar.com"
                  css=""
                  {...email}
                />
              </Label>

              <Label className="mt-4">
                <span>Password</span>
                <Input
                  valid={error ? false : undefined}
                  className="mt-1"
                  type="password"
                  placeholder="•••••••••"
                  css=""
                  {...password}
                />
              </Label>

              <div className="flex justify-between items-center mt-4">
                <Label className="inline-flex items-center">
                  <Input
                    type="checkbox"
                    css=""
                    className="text-purple-600 dark:text-purple-400"
                    onChange={() => setRemember(!remember)}
                    checked={remember}
                  />
                  <span className="mx-2 text-sm font-medium">Remember Me</span>
                </Label>

                <Link
                  className="text-sm font-medium text-purple-600 dark:text-purple-400 hover:underline"
                  to="/"
                >
                  Forgot your password?
                </Link>
              </div>

              <Button className="mt-4 w-full" disabled={loading} onClick={handleLogin}>
                {loading ? 'Loading...' : 'Login'}
              </Button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
