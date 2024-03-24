// index.tsx
import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './styles/index.css'; // Your Tailwind CSS file
import { getApiBaseUrl, setApiBaseUrl } from '@communest/shared';
setApiBaseUrl(import.meta.env.VITE_API_BASE_URL);

console.log('env => ', import.meta.env.VITE_API_BASE_URL);

console.log('env 2 =>', getApiBaseUrl());
const container = document.getElementById('root')!;
const root = createRoot(container); // createRoot instead of ReactDOM.render
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
