import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from "tailwindcss";
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    open: true, // Automatically open the browser when the server starts
  },
  css: {
    postcss: {
      plugins: [tailwindcss()],
    },
  },
})
