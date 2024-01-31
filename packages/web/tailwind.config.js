/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'media',
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: {
          light: '#F3A712', // Warm yellow
          DEFAULT: '#E9A115', // Deep yellow
          dark: '#DB9200', // Mustard yellow
        },
        secondary: {
          light: '#F2CC8F', // Soft orange
          DEFAULT: '#E4B363', // Earthy orange
          dark: '#D9A53D', // Deep orange
        },
        accent: {
          light: '#81B29A', // Teal
          DEFAULT: '#709A89', // Dark teal
          dark: '#5E8273', // Deeper teal
        },
        background: {
          light: '#FEFBF6', // Off-white
          dark: '#2B2D42', // Dark blue-gray
        },
        textLight: {
          light: '#2B2D42', // Dark blue-gray
          dark: '#FEFBF6', // Off-white
        },
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
