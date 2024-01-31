import React from 'react';
import { NavLink } from 'react-router-dom';

interface HeaderProps {
  darkMode: boolean;
  toggleDarkMode: () => void;
}

const Header: React.FC<HeaderProps> = ({ darkMode, toggleDarkMode }) => {
  return (
    <header
      className={`bg-background shadow-md ${darkMode ? 'text-textLight' : 'text-primary'}`}
    >
      <div className="container mx-auto flex justify-between items-center py-4 px-6">
        {/* Logo and Title */}
        <div className="flex items-center">
          <img src="/logo.svg" alt="Communest Logo" className="h-12 mr-3" />
          <h1 className="text-xl font-bold">Communest</h1>
        </div>

        {/* Navigation and Mode Toggle */}
        <div className="flex items-center">
          <nav>
            <ul className="flex space-x-6 mr-6">
              <li>
                <NavLink
                  to="/login"
                  className={({ isActive }) => (isActive ? 'underline' : '')}
                >
                  Login
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="/signup"
                  className={({ isActive }) => (isActive ? 'underline' : '')}
                >
                  Sign Up
                </NavLink>
              </li>
            </ul>
          </nav>
          <button
            onClick={toggleDarkMode}
            className="px-4 py-2 bg-secondary text-background rounded"
          >
            {darkMode ? 'Light' : 'Dark'} Mode
          </button>
        </div>
      </div>
    </header>
  );
};

export default Header;
