import React from 'react';
import { NavLink } from 'react-router-dom';
import { useTheme } from '../App'; // Update this path to the actual location of your App component or where useTheme is defined
import CommunestLogo from './icons/CommunestLogo';

const Header: React.FC = () => {
  const { darkMode, toggleDarkMode } = useTheme();

  return (
    <header
      style={{
        backgroundColor: darkMode ? '#121212' : '#F5F5F5', // Assuming these are your theme's dark and light background colors
        color: darkMode ? '#E0E0E0' : '#333333', // Assuming these are your theme's dark and light text colors
        boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
        transition: 'background-color 0.3s, color 0.3s', // Smooth transition for theme change
      }}
      className="shadow-md"
    >
      <div className="container mx-auto flex justify-between items-center py-4 px-6">
        {/* Logo and Title */}
        <div className="flex items-center">
          <div className="h-12 mr-3">
            <CommunestLogo className="logo" />
          </div>
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
            style={{
              backgroundColor: '#FFB562', // Assuming this is your secondary color
              color: '#F5F5F5', // Assuming this is your light background color, used here for contrast
              borderRadius: '8px', // Example of using a theme-based style
            }}
            className="px-4 py-2 rounded"
          >
            {darkMode ? 'Light' : 'Dark'} Mode
          </button>
        </div>
      </div>
    </header>
  );
};

export default Header;
