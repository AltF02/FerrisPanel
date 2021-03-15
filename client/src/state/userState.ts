import { useState } from 'react';
import { createContainer } from 'unstated-next';
import { State, Api } from '../types';

const axios = require('axios').default;

function userState(): State {
  const [authenticated, setAuthenticated] = useState<boolean | undefined>(undefined);
  const [loading, setLoading] = useState(false);
  // eslint-disable-next-line no-unused-vars
  const [userData, setUserData] = useState<Api.User | undefined>(undefined);

  function logout() {
    return axios
      .post('/auth/logout')
      .then((response: any) => {
        if (response.status === 200) {
          setAuthenticated(false);
        } else {
          console.error('Unable to log out', response.status);
        }
      })
      .catch((err: any) => console.error(err));
  }

  function authenticate(id: string, password: string, remember: boolean): Promise<boolean> {
    console.log(remember);
    setLoading(true);
    const data = JSON.stringify({ id, password });

    const headers = {
      'Content-Type': 'application/json',
    };

    return axios
      .post('/auth/login', data, { headers })
      .then((response: any) => {
        const success = response.status === 200;

        if (success) {
          setUserData(response.data);
        }

        setAuthenticated(success);
        setLoading(false);
        return !success;
      }).catch((err: any) => {
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
    logout,
    loading,
  };
}

export function useUserState() {
  return createContainer(userState);
}

export default useUserState();
