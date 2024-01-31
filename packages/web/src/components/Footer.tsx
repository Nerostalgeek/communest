import React from 'react';

const Footer: React.FC = () => {
  return (
    <footer className="bg-background text-gray-600 body-font">
      <div className="container px-5 py-8 mx-auto flex items-center sm:flex-row flex-col">
        {/* Logo and Title */}
        <div className="flex title-font font-medium items-center md:justify-start justify-center text-gray-900">
          {/* Replace 'logo.svg' with your logo's path */}
          <img src="/logo.svg" alt="Communest Logo" className="h-12" />
          <span className="ml-3 text-xl">Communest</span>
        </div>

        {/* About Section */}
        <p className="text-sm text-gray-500 sm:ml-6 sm:mt-0 mt-4">
          © {new Date().getFullYear()} Communest —
          <a
            href="https://github.com/Communest"
            className="text-gray-600 ml-1"
            rel="noopener noreferrer"
            target="_blank"
          >
            @CommunestGitHub
          </a>
        </p>

        {/* Social Links */}
        <span className="inline-flex sm:ml-auto sm:mt-0 mt-4 justify-center sm:justify-start">
          <a
            className="text-gray-500"
            href="https://facebook.com/Communest"
            target="_blank"
            rel="noopener noreferrer"
          >
            <svg
              fill="currentColor"
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              className="h-5 w-5"
              viewBox="0 0 24 24"
            >
              {/* Facebook Icon */}
            </svg>
          </a>
          <a
            className="ml-3 text-gray-500"
            href="https://twitter.com/Communest"
            target="_blank"
            rel="noopener noreferrer"
          >
            <svg
              fill="currentColor"
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              className="h-5 w-5"
              viewBox="0 0 24 24"
            >
              {/* Twitter Icon */}
            </svg>
          </a>
          {/* Add more social links as needed */}
        </span>

        {/* Contact Info or Additional Links */}
        {/* Add here if needed */}
      </div>
    </footer>
  );
};

export default Footer;
