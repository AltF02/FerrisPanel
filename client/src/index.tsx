import ReactDOM from 'react-dom';
import './index.css';
import React, { Suspense } from 'react';
import { Windmill } from '@windmill/react-ui';
import App from './App';
import Fallback from './components/Fallback';

ReactDOM.render(
  <Windmill usePreferences dark>
    <Suspense fallback={<Fallback />}>
      <App />
    </Suspense>
  </Windmill>,
  document.getElementById('root'),
);
