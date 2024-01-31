/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'media',
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: '#007bff',
        secondary: '#ff6b6b',
        accent: '#adb5bd',
        background: {
          light: '#e7f5ff',
          dark: '#343a40',
        },
        textLight: '#343a40',
        textDark: '#f8f9fa',
      },
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
