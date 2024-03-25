// index.tsx
import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './styles/index.css'; // Your Tailwind CSS file
import { setApiBaseUrl } from '@communest/shared';
setApiBaseUrl(import.meta.env.VITE_API_BASE_URL);

console.log('env => ', import.meta.env.VITE_API_BASE_URL);
const container = document.getElementById('root')!;
const root = createRoot(container); // createRoot instead of ReactDOM.render
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
