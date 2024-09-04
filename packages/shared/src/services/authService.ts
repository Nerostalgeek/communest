import axios from 'axios';
import { getApiBaseUrl } from '../utils/config';

interface LoginRequest {
  email: string;
  password: string;
}

interface AuthResponse {
  token: string;
}

export const login = async ({
  email,
  password,
}: LoginRequest): Promise<AuthResponse> => {
  try {
    const API_BASE_URL = getApiBaseUrl() + '/api';

    const response = await axios.post<AuthResponse>(
      `${API_BASE_URL}/v1/auth/login`,
      { email, password }
    );
    return response.data;
  } catch (error: unknown) {
    if (axios.isAxiosError(error)) {
      console.error('Login error:', error.response);
      throw error;
    } else {
      console.error('Unexpected error:', error);
      throw new Error('An unexpected error occurred');
    }
  }
};
interface RegisterRequest {
  email: string;
  password: string;
}

interface RefreshTokenRequest {
  token: string;
}

interface RegisterResponse {
  user: {
    id: string;
    email: string;
  };
  token: string;
}

interface LogoutResponse {
  message: string;
}

interface RefreshTokenResponse {
  token: string;
}

export const register = async ({
  email,
  password,
}: RegisterRequest): Promise<RegisterResponse> => {
  const API_BASE_URL = getApiBaseUrl() + '/api';

  const response = await axios.post<RegisterResponse>(
    `${API_BASE_URL}/v1/auth/register`,
    { email, password }
  );
  return response.data;
};


export const logout = async (): Promise<LogoutResponse> => {
  const API_BASE_URL = getApiBaseUrl() + '/api';

  const response = await axios.post<LogoutResponse>(
    `${API_BASE_URL}/v1/auth/logout`,
    {}
  );
  return response.data;
};


export const refreshToken = async ({
  token,
}: RefreshTokenRequest): Promise<RefreshTokenResponse> => {
  const API_BASE_URL = getApiBaseUrl() + '/api';

  const response = await axios.post<RefreshTokenResponse>(
    `${API_BASE_URL}/v1/auth/refresh-token`,
    { token }
  );
  return response.data;
};

export type {
  LoginRequest,
  AuthResponse,
  RegisterRequest,
  RegisterResponse,
  LogoutResponse,
  RefreshTokenRequest,
  RefreshTokenResponse,
};
