import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
// Import your components
import Header from './components/Header';
import Footer from './components/Footer';
import HomePage from './Pages/HomePage';
import AboutPage from './Pages/AboutPage';
// Import other pages or components as needed

const App: React.FC = () => {
  return (
    <div className="min-h-screen flex flex-col text-gray-800">
      <header className="bg-blue-600 text-white text-xl p-4">
        Header Content
      </header>
      <main className="flex-grow p-4">
        <h1 className="text-2xl font-bold">Welcome to Communest</h1>
        <p className="mt-2">Your community engagement platform.</p>
        <Router>
          <div className="App">
            <Header />
            <main>
              <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="/about" element={<AboutPage />} />
                {/* Define more routes as needed */}
              </Routes>
            </main>
            <Footer />
          </div>
        </Router>
      </main>
      <footer className="bg-gray-200 text-center p-4">
        © 2024 Communest. All rights reserved.
      </footer>
    </div>
  );
};

export default App;
