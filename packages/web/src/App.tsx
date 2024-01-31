import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
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
  // Initialize darkMode state based on user preference or system preference
  const [darkMode, setDarkMode] = useState<boolean>(() => {
    const userPrefersDark = localStorage.getItem('darkMode');
    // User preference found in localStorage
    if (userPrefersDark !== null) {
      return userPrefersDark === 'true';
    }
    // Fallback to system preference
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  });

  // Effect to apply or remove the dark mode class on the body element
  useEffect(() => {
    // Check for user preference or system theme
    const darkMode =
      localStorage.getItem('darkMode') === 'true' ||
      window.matchMedia('(prefers-color-scheme: dark)').matches;

    if (darkMode) {
      document.body.classList.add('dark');
    } else {
      document.body.classList.remove('dark');
    }
  }, []);

  // Toggle dark mode
  const toggleDarkMode = () => setDarkMode(!darkMode);

  return (
    <Router>
      <div
        className={`${
          darkMode
            ? 'dark bg-background-dark text-textLight'
            : 'bg-background-light text-gray-800'
        } min-h-screen flex flex-col`}
      >
        <Header darkMode={darkMode} toggleDarkMode={toggleDarkMode} />
        <main className="flex-grow p-4">
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
