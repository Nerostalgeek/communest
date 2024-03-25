// In shared/utils/config.ts
let apiBaseUrl = '';

export const setApiBaseUrl = (url: string) => {
  apiBaseUrl = url;
};

export const getApiBaseUrl = () => {
  console.log('inside the functio itself => ', apiBaseUrl);
  return apiBaseUrl;
};
