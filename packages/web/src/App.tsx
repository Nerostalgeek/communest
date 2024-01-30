import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
// Import your components
import Header from './components/Header';
import Footer from './components/Footer';
import HomePage from './pages/HomePage';
import AboutPage from './pages/AboutPage';
import LoginPage from './pages/auth/LoginPage';
import SignupPage from './pages/auth/SignupPage';
// Import other pages or components as needed

const App: React.FC = () => {
  return (
    <div className="min-h-screen flex flex-col text-gray-800">
      <Router>
        <Header />
        <div className="App">
          <main className="flex-grow p-4">
            <Routes>
              <Route path="/" element={<HomePage />} />
              <Route path="/login" element={<LoginPage />} />
              <Route path="/signup" element={<SignupPage />} />
              <Route path="/about" element={<AboutPage />} />
              {/* Define more routes as needed */}
            </Routes>
          </main>
          <Footer />
        </div>
      </Router>
    </div>
  );
};

export default App;
