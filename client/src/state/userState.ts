import { useState } from 'react';
import { createContainer } from 'unstated-next';

interface State {
  authenticated?: boolean,
  // eslint-disable-next-line no-unused-vars
  authenticate: (id: string, password: string) => Promise<boolean>
  loading: boolean,
}

function userState(): State {
  const [authenticated, setAuthenticated] = useState<boolean | undefined>(undefined);
  const [loading, setLoading] = useState(false);

  function authenticate(id: string, password: string): Promise<boolean> {
    setLoading(true);
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ id, password }),
    };

    return fetch('/auth/login', requestOptions)
      .then((response) => {
        const success = response.status === 200;
        setAuthenticated(success);
        setLoading(false);
        return !success;
      }).catch((err) => {
        // eslint-disable-next-line no-console
        console.error(err);

        setAuthenticated(false);
        setLoading(false);
        return true;
      });
  }

  return {
    authenticated,
    authenticate,
    loading,
  };
}

export function useUserState() {
  return createContainer(userState);
}

export default useUserState();
