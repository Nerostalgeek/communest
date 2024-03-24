// In shared/utils/config.ts
let apiBaseUrl = '';

export const setApiBaseUrl = (url: string) => {
  apiBaseUrl = url;
};

export const getApiBaseUrl = () => apiBaseUrl;
