import { useState } from 'react';
import { createContainer } from 'unstated-next';
import { AxiosResponse } from 'axios';
import { State, Api } from '../types';

const axios = require('axios').default;

function userState(): State {
  const [authenticated, setAuthenticated] = useState<boolean | undefined>(undefined);
  const [loading, setLoading] = useState(false);
  const [userData, setUserData] = useState<Api.User | undefined>(undefined);

  function fetch(): Promise<boolean> {
    return axios.get('/api/users/@me').then((response: AxiosResponse) => {
      if (response.status === 200) {
        setUserData(response.data);
        setAuthenticated(true);
        return true;
      }
      return false;
    }).catch(() => false);
  }

  function logout() {
    return axios
      .post('/auth/logout')
      .then((response: AxiosResponse) => {
        if (response.status === 200) {
          setAuthenticated(false);
        }
      })
      .catch(() => false);
  }

  function authenticate(update: boolean, data: Api.AuthData | undefined): Promise<boolean> {
    setLoading(true);
    const jsonData = JSON.stringify(data);

    const headers = {
      'Content-Type': 'application/json',
    };

    return axios
      .post('/auth/login', jsonData, { headers })
      .then((response: AxiosResponse) => {
        const success = response.status === 200;

        if (success) {
          setUserData(response.data);
        }

        setAuthenticated(success);
        setLoading(false);
        return !success;
      }).catch(() => {
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
    fetch,
    userData,
  };
}

export function useUserState() {
  return createContainer(userState);
}

export default useUserState();
