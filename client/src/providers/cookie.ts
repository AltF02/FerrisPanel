import { createContainer } from 'unstated-next';
import { useEffect } from 'react';
import { useHistory, useLocation } from 'react-router';
import { UserState } from '../state';

function Cookie() {
  const { authenticated, fetch } = UserState.useContainer();
  const history = useHistory();
  const location = useLocation();

  useEffect(() => {
    if (!authenticated) {
      fetch().then((success: boolean) => {
        if (success) {
          history.push('/app');
        } else if (!location.pathname.startsWith('/login')) {
          history.push('/login');
        }
      });
    }
  }, [history]);
}

export function useCookie() {
  return createContainer(Cookie);
}

export default useCookie();
