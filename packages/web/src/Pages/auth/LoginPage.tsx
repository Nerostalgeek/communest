import { useState } from 'react';
import { useTheme } from '../../App';
import { theme } from '@communest/shared';
const LoginPage: React.FC = () => {
  const { darkMode } = useTheme();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = async (event: React.FormEvent) => {
    event.preventDefault();
    // Call the login service using the email and password state
    // You'll need to replace this with your actual http call logic
    console.log('Login Attempt:', { email, password });
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
      <form onSubmit={handleLogin} style={{ width: '300px' }}>
        <h1>Login</h1>
        <input
          type="email"
          placeholder="Email"
          required
          value={email}
          onChange={(e) => setEmail(e.target.value)}
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
          Log In
        </button>
      </form>
    </div>
  );
};

export default LoginPage;
