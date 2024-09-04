type ColorScheme = {
  light: string;
  dark: string;
};

interface ThemeColors {
  primary: ColorScheme;
  secondary: ColorScheme;
  accent: ColorScheme;
  success: ColorScheme;
  warning: ColorScheme;
  error: ColorScheme;
  info: ColorScheme;
  neutral: ColorScheme;
  background: ColorScheme;
  text: ColorScheme;
}

interface ThemeSpacing {
  xs: number;
  sm: number;
  md: number;
  lg: number;
  xl: number;
}

interface ThemeTypography {
  fontFamily: string;
  fontSize: {
    small: string;
    medium: string;
    large: string;
    xlarge: string;
  };
  fontWeight: {
    normal: number;
    medium: number;
    bold: number;
  };
}

interface ThemeBreakpoints {
  sm: string;
  md: string;
  lg: string;
  xl: string;
}

interface ThemeShadows {
  default: string;
  medium: string;
  large: string;
}

interface ThemeBorders {
  radius: {
    small: string;
    default: string;
    large: string;
    circle: string;
  };
}

interface ThemeTransitions {
  default: string;
}

const utils = {
  convertHexToRGB: (hex: string): string => {
    let r = 0,
      g = 0,
      b = 0;

    // 3 digits
    if (hex.length === 4) {
      r = parseInt(hex[1] + hex[1], 16);
      g = parseInt(hex[2] + hex[2], 16);
      b = parseInt(hex[3] + hex[3], 16);
    }
    // 6 digits
    else if (hex.length === 7) {
      r = parseInt(hex[1] + hex[2], 16);
      g = parseInt(hex[3] + hex[4], 16);
      b = parseInt(hex[5] + hex[6], 16);
    }
    return `rgb(${r}, ${g}, ${b})`;
  },

  lightenColor: (hex: string, percent: number): string => {
    const rgb = utils.convertHexToRGB(hex).match(/\d+/g);
    if (!rgb) {
      return hex; // Return original hex if conversion fails
    }
    const [r, g, b] = rgb.map(Number);
    const increase = (color: number) =>
      Math.min(255, Math.floor(color + (255 - color) * (percent / 100)));
    return `rgb(${increase(r)}, ${increase(g)}, ${increase(b)})`;
  },

  darkenColor: (hex: string, percent: number): string => {
    const rgb = utils.convertHexToRGB(hex).match(/\d+/g);
    if (!rgb) {
      return hex;
    }
    const [r, g, b] = rgb.map(Number);
    const decrease = (color: number) =>
      Math.max(0, Math.floor(color * (1 - percent / 100)));
    return `rgb(${decrease(r)}, ${decrease(g)}, ${decrease(b)})`;
  },
};

export interface Theme {
  colors: ThemeColors;
  spacing: ThemeSpacing;
  typography: ThemeTypography;
  breakpoints: ThemeBreakpoints;
  shadows: ThemeShadows;
  borders: ThemeBorders;
  transitions: ThemeTransitions;


  utils: typeof utils;
}

export const theme: Theme = {
  colors: {
    primary: { light: '#407BFF', dark: '#609FFF' },
    secondary: { light: '#FFB562', dark: '#FFA85C' },
    accent: { light: '#FF7851', dark: '#FF8965' },
    success: { light: '#4CAF50', dark: '#66BB6A' },
    warning: { light: '#FFC107', dark: '#FFCA28' },
    error: { light: '#F44336', dark: '#EF5350' },
    info: { light: '#2196F3', dark: '#42A5F5' },
    neutral: { light: '#9E9E9E', dark: '#BDBDBD' },
    background: { light: '#F5F5F5', dark: '#121212' },
    text: { light: '#333333', dark: '#E0E0E0' },
  },
  spacing: {
    xs: 4,
    sm: 8,
    md: 16,
    lg: 32,
    xl: 64,
  },
  typography: {
    fontFamily: "'Roboto', sans-serif",
    fontSize: {
      small: '0.875rem',
      medium: '1rem',
      large: '1.25rem',
      xlarge: '1.5rem',
    },
    fontWeight: {
      normal: 400,
      medium: 500,
      bold: 700,
    },
  },
  breakpoints: {
    sm: '640px',
    md: '768px',
    lg: '1024px',
    xl: '1280px',
  },
  shadows: {
    default: '0px 2px 4px rgba(0, 0, 0, 0.1)',
    medium: '0px 4px 6px rgba(0, 0, 0, 0.1)',
    large: '0px 10px 15px rgba(0, 0, 0, 0.1)',
  },
  borders: {
    radius: {
      small: '4px',
      default: '8px',
      large: '16px',
      circle: '50%',
    },
  },
  transitions: {
    default: 'background-color 0.3s, color 0.3s',
  },
  utils,
};
