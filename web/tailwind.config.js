module.exports = {
  darkMode: 'media',
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      // Custom color scheme
      colors: {
        primary: '#005f73', // Teal Blue
        secondary: '#e29578', // Soft Orange
        accent: '#e63946', // Bright Red
        darkAccent: '#a8dadc', // Soft Cyan
        background: '#f1faee', // Light Ivory
        darkBackground: '#1d3557', // Dark Navy
        gray: '#6d6875', // Neutral Gray
      },
      // Additional customizations can be made here
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
