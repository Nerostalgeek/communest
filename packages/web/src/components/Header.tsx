import React from 'react';
import { NavLink } from 'react-router-dom';
const Header: React.FC = () => {
  return (
    <header className="bg-blue-600 text-white p-4 flex justify-between items-center">
      <h1 className="text-xl font-semibold">Communest</h1>
      <nav>
        <ul className="flex space-x-4">
          <li>
            <NavLink
              to="/login"
              className={({ isActive }) =>
                isActive ? 'underline text-blue-300' : ''
              }
            >
              Login
            </NavLink>
          </li>
          <li>
            <NavLink
              to="/signup"
              className={({ isActive }) =>
                isActive ? 'underline text-blue-300' : ''
              }
            >
              Sign Up
            </NavLink>
          </li>
        </ul>
      </nav>
    </header>
  );
};

export default Header;
