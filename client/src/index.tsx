import ReactDOM from 'react-dom';
import './index.css';
import React from 'react';
import { Windmill } from '@windmill/react-ui';
import App from './App';

ReactDOM.render(
  <Windmill dark>
    <App />
  </Windmill>,
  document.getElementById('root'),
);
