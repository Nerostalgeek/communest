import React, { useState, useEffect } from 'react';
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

  return (
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
        <Header darkMode={darkMode} toggleDarkMode={toggleDarkMode} />
        <main style={{ flexGrow: 1, padding: '16px' }}>
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
  );
};

export default App;
