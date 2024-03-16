type ColorObject = {
  DEFAULT: string;
  dark: string;
};

interface Theme {
  colors: {
    primary: ColorObject;
    secondary: ColorObject;
    accent: ColorObject;
    success: ColorObject;
    warning: ColorObject;
    error: ColorObject;
    info: ColorObject;
    neutral: ColorObject;
    background: ColorObject;
    text: ColorObject;
  };
  spacing: {
    small: number;
    medium: number;
    large: number;
    // ... other spacings
  };
  // ... other theme properties
}

export const theme: Theme = {
  colors: {
    primary: {
      DEFAULT: '#d04848', // Rouge foncé
      dark: '#a83838', // Rouge plus foncé
    },
    secondary: {
      DEFAULT: '#f3b95f', // Jaune orangé
      dark: '#c9934f', // Jaune orangé plus foncé
    },
    accent: {
      DEFAULT: '#fde767', // Jaune clair
      dark: '#c9b856', // Jaune plus foncé
    },
    success: {
      DEFAULT: '#6895d2', // Bleu moyen
      dark: '#5073b8', // Bleu plus foncé
    },
    warning: {
      DEFAULT: '#F6AD55', // Orange
      dark: '#DD6B20', // Orange plus foncé
    },
    error: {
      DEFAULT: '#F56565', // Rouge
      dark: '#C53030', // Rouge plus foncé
    },
    info: {
      DEFAULT: '#63B3ED', // Bleu clair
      dark: '#3182CE', // Bleu plus foncé
    },
    neutral: {
      DEFAULT: '#B0BEC5', // Gris neutre
      dark: '#78909C', // Gris plus foncé
    },
    background: {
      DEFAULT: '#FFF9F0', // Crème léger pour l'arrière-plan
      dark: '#3A3F4B', // Gris bleuté foncé pour le fond sombre
    },
    text: {
      DEFAULT: '#333333', // Gris foncé pour le texte
      dark: '#EDEDED', // Gris clair pour le texte en mode sombre
    },
  },
  spacing: {
    small: 8,
    medium: 16,
    large: 32,
    // ... autres espacements
  },
  // ... other theme values
};
