import axios from 'axios';

// Define the request and response types for better type checking and IntelliSense support
interface LoginRequest {
  email: string;
  password: string;
}

interface AuthResponse {
  token: string; // Adjust according to your actual API response
  // Add any other relevant fields returned by your API
}

const API_BASE_URL: string = process.env.REACT_APP_API_BASE_URL || ''; // Ensure this is set in your .env file

export const login = async ({
  email,
  password,
}: LoginRequest): Promise<AuthResponse> => {
  try {
    const response = await axios.post<AuthResponse>(
      `${API_BASE_URL}/auth/login`,
      { email, password }
    );
    return response.data;
  } catch (error: unknown) {
    if (axios.isAxiosError(error)) {
      console.error('Login error:', error.response);
      // Re-throw the error for handling by the caller
      throw error;
    } else {
      console.error('Unexpected error:', error);
      // Handle or throw a generic error
      throw new Error('An unexpected error occurred');
    }
  }
};

// Additional interfaces for registration and possibly token refresh
interface RegisterRequest {
  email: string;
  password: string;
  // Add any other fields required for registration
}

interface RefreshTokenRequest {
  token: string;
}

// Assuming your API returns similar responses for these actions
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

// Registration endpoint
export const register = async ({
  email,
  password,
}: RegisterRequest): Promise<RegisterResponse> => {
  const response = await axios.post<RegisterResponse>(
    `${API_BASE_URL}/auth/register`,
    { email, password }
  );
  return response.data;
};

// Logout endpoint
export const logout = async (): Promise<LogoutResponse> => {
  const response = await axios.post<LogoutResponse>(
    `${API_BASE_URL}/auth/logout`,
    {}
  );
  return response.data;
};

// Token refresh endpoint
export const refreshToken = async ({
  token,
}: RefreshTokenRequest): Promise<RefreshTokenResponse> => {
  const response = await axios.post<RefreshTokenResponse>(
    `${API_BASE_URL}/auth/refresh-token`,
    { token }
  );
  return response.data;
};
