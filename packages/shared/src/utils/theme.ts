type ColorScheme = {
  light: string;
  dark: string;
};

interface ThemeColors {
  primary: ColorScheme; // Main brand color
  secondary: ColorScheme; // Supportive brand color
  accent: ColorScheme; // Accent color for call-to-actions
  success: ColorScheme; // Success messages and indicators
  warning: ColorScheme; // Warning messages and indicators
  error: ColorScheme; // Error messages and indicators
  info: ColorScheme; // Informational messages and indicators
  neutral: ColorScheme; // Neutral colors for UI elements
  background: ColorScheme; // Background color
  text: ColorScheme; // Text color
}

interface ThemeSpacing {
  xs: number;
  sm: number;
  md: number;
  lg: number;
  xl: number;
}

interface Theme {
  colors: ThemeColors;
  spacing: ThemeSpacing;
}

export const theme: Theme = {
  colors: {
    primary: { light: '#4A90E2', dark: '#1E5A99' },
    secondary: { light: '#A3D4F7', dark: '#276F9F' },
    accent: { light: '#E2B13C', dark: '#F29F05' },
    success: { light: '#50C878', dark: '#3D8B37' },
    warning: { light: '#FFD700', dark: '#CFA204' },
    error: { light: '#FF6347', dark: '#D8462F' },
    info: { light: '#3B9AD9', dark: '#307BA0' },
    neutral: { light: '#F5F5F5', dark: '#2A2E35' },
    background: { light: '#FFFFFF', dark: '#121417' },
    text: { light: '#313639', dark: '#E4E6E9' },
  },
  spacing: {
    xs: 4,
    sm: 8,
    md: 16,
    lg: 32,
    xl: 64,
  },
};
