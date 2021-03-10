import React, { useState } from 'react';
import { Button, Badge, Input } from '@windmill/react-ui';
import { RouteComponentProps } from 'react-router-dom';
import useFormInput from '../utils/form';

interface IProps extends RouteComponentProps {
}

export default function LoginForm(props: IProps) {
  const email = useFormInput('');
  const password = useFormInput('');
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(false);

  const handleLogin = () => {
    setLoading(true);
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ id: email.value, password: password.value }),
    };
    fetch('/auth/login', requestOptions)
      .then((response) => {
        if (response.status !== 200) {
          response.json().then((data) => setError(data.info));
          setLoading(false);
        } else {
          props.history.push('/dashboard');
        }
      });
  };

  return (
    <div>
      <h1 className="mb-4 text-xl font-semibold text-gray-700 dark:text-gray-200">Login</h1>
      <div>
        <Input type="text" {...email} autoComplete="new-password" css="" placeholder="foo@bar.com" />
      </div>
      <div style={{ marginTop: 10 }}>
        <Input type="password" {...password} autoComplete="new-password" css="" placeholder="•••••••" />
      </div>
      {error && (
      <>
        <Badge type="danger">{error}</Badge>
      </>
      )}
      <br />
      <Button onClick={handleLogin} disabled={loading}>{loading ? 'Loading...' : 'Login'}</Button>
      <br />
    </div>
  );
}
