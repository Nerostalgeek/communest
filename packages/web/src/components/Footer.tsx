import React from 'react';

const Footer: React.FC = () => {
  return (
    <footer className="bg-background text-gray-600 body-font">
      <div className="container px-5 py-8 mx-auto flex items-center sm:flex-row flex-col">
        <div className="flex title-font font-medium items-center md:justify-start justify-center text-gray-900">
          <img src="/logo.svg" alt="Communest Logo" className="h-12" />
          <span className="ml-3 text-xl">Communest</span>
        </div>

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
            </svg>
          </a>
        </span>
      </div>
    </footer>
  );
};

export default Footer;
