import ReactDOM from 'react-dom';
import './index.css';
import React, { Suspense } from 'react';
import { Windmill } from '@windmill/react-ui';
import App from './App';

ReactDOM.render(
  <Windmill usePreferences dark>
    <Suspense fallback={<div>Loading... </div>}>
      <App />
    </Suspense>
  </Windmill>,
  document.getElementById('root'),
);
