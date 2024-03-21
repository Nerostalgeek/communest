import React, { createContext, useContext, useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { theme } from '@communest/shared';

// Import your components
import Header from './components/Header';
import Footer from './components/Footer';
import HomePage from './pages/HomePage';
import AboutPage from './pages/AboutPage';
import LoginPage from './pages/auth/LoginPage';
import SignupPage from './pages/auth/SignupPage';
import WelcomePage from './pages/WelcomePage';
// Import other pages or components as needed

// Theme context to provide access to theme settings and toggle function
interface ThemeContextType {
  darkMode: boolean;
  toggleDarkMode: () => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

// Custom hook to use theme context
export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}

const App: React.FC = () => {
  const [darkMode, setDarkMode] = useState<boolean>(() => {
    const userPrefersDark = localStorage.getItem('darkMode');
    if (userPrefersDark !== null) {
      return userPrefersDark === 'true';
    }
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  });

  useEffect(() => {
    if (darkMode) {
      document.body.classList.add('dark');
    } else {
      document.body.classList.remove('dark');
    }
  }, [darkMode]);

  const toggleDarkMode = () => {
    setDarkMode(!darkMode);
    localStorage.setItem('darkMode', String(!darkMode));
  };

  const themeContextValue = { darkMode, toggleDarkMode };

  return (
    <ThemeContext.Provider value={themeContextValue}>
      <Router>
        <div
          style={{
            backgroundColor: darkMode
              ? theme.colors.background.dark
              : theme.colors.background.light,
            color: darkMode ? theme.colors.text.dark : theme.colors.text.light,
            minHeight: '100vh',
            display: 'flex',
            flexDirection: 'column',
          }}
        >
          <Header />
          <main style={{ flexGrow: 1, padding: theme.spacing.md }}>
            <Routes>
              <Route path="/" element={<WelcomePage />} />
              <Route path="/home" element={<HomePage />} />
              <Route path="/login" element={<LoginPage />} />
              <Route path="/signup" element={<SignupPage />} />
              <Route path="/about" element={<AboutPage />} />
              {/* Define more routes as needed */}
            </Routes>
          </main>
          <Footer />
        </div>
      </Router>
    </ThemeContext.Provider>
  );
};

export default App;
