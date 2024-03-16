/** @type {import('tailwindcss').Config} */

import { theme } from '@communest-monorepo/shared/dist/theme';

export default {
  darkMode: 'media',
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: theme.colors,
    },
  },
  variants: {
    extend: {
      // Extend Tailwind's default variants as needed
    },
  },
  plugins: [
    // Add Tailwind plugins here if needed
  ],
};
