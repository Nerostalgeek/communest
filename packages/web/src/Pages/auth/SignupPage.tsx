import React, { useState } from 'react';
import { useTheme } from '../../App';

import { theme } from '@communest/shared';
const SignupPage: React.FC = () => {
  const { darkMode } = useTheme();
  const [lastName, setLastName] = useState('');
  const [firstName, setFirstName] = useState('');
  const [email, setEmail] = useState('');
  const [phoneNumber, setPhoneNumber] = useState('');
  const [password, setPassword] = useState('');

  const handleSignup = async (event: React.FormEvent) => {
    event.preventDefault();
    // Implement your sign up logic here, calling the relevant API with these details
    console.log('Signup Attempt:', {
      lastName,
      firstName,
      email,
      phoneNumber,
      password,
    });
    // TODO: Replace the console.log with your sign up logic
  };

  const inputStyle: React.CSSProperties = {
    width: '100%',
    padding: '10px',
    margin: '10px 0',
    borderRadius: '4px',
    border: `1px solid ${darkMode ? '#FFFFFF' : '#CCCCCC'}`,
    backgroundColor: darkMode
      ? theme.colors.background.dark
      : theme.colors.background.light,
    color: darkMode ? theme.colors.text.dark : theme.colors.text.light,
  };

  const containerStyle: React.CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    height: '100vh',
    textAlign: 'center',
    backgroundColor: darkMode
      ? theme.colors.background.dark
      : theme.colors.background.light,
    color: darkMode ? theme.colors.text.dark : theme.colors.text.light,
  };

  const buttonStyle: React.CSSProperties = {
    backgroundColor: theme.colors.primary.light,
    color: theme.colors.text.light,
    padding: '10px 20px',
    margin: '20px 0',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
    fontSize: '1rem',
    fontWeight: 'bold',
  };
  return (
    <div style={containerStyle}>
      <form onSubmit={handleSignup} style={{ width: '300px' }}>
        <h1>Create Your Account</h1>
        <input
          type="text"
          placeholder="Last Name"
          required
          value={lastName}
          onChange={(e) => setLastName(e.target.value)}
          style={inputStyle}
        />
        <input
          type="text"
          placeholder="First Name"
          required
          value={firstName}
          onChange={(e) => setFirstName(e.target.value)}
          style={inputStyle}
        />
        <input
          type="email"
          placeholder="Email"
          required
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          style={inputStyle}
        />
        <input
          type="tel"
          placeholder="Phone Number (optional)"
          value={phoneNumber}
          onChange={(e) => setPhoneNumber(e.target.value)}
          style={inputStyle}
        />
        <input
          type="password"
          placeholder="Password"
          required
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          style={inputStyle}
        />
        <button type="submit" style={buttonStyle}>
          Sign Up
        </button>
      </form>
    </div>
  );
};

export default SignupPage;
