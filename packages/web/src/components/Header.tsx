import React from 'react';
import { NavLink } from 'react-router-dom';
import { useTheme } from '../App';
import CommunestLogo from './icons/CommunestLogo';

const Header: React.FC = () => {
  const { darkMode, toggleDarkMode, theme } = useTheme();

  return (
    <header
      style={{
        backgroundColor: darkMode
          ? theme.colors.background.dark
          : theme.colors.background.light,
        color: darkMode
          ? theme.colors.primary.dark
          : theme.colors.primary.light,
        boxShadow: theme.shadows.default,
        transition: theme.transitions.default,
      }}
      className="shadow-md"
    >
      <div className="container mx-auto flex justify-between items-center py-4 px-6">

        <div className="flex items-center">
          <div className="h-12 mr-3">
            <CommunestLogo className="logo" />
          </div>
          <h1 className="text-xl font-bold">Communest</h1>
        </div>


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
              backgroundColor: '#FFB562',
              color: '#F5F5F5',
              borderRadius: '8px',
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
