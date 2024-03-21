import { useTheme } from '../App';
import { theme } from '@communest/shared';
import { NavLink } from 'react-router-dom';

const WelcomePage: React.FC = () => {
  const { darkMode } = useTheme();

  // Utilize the theme for styles
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
    transition: 'all 0.3s',
  };

  const buttonStyle: React.CSSProperties = {
    padding: '10px 20px',
    margin: '20px',
    borderRadius: theme.borders.radius.default,
    backgroundColor: theme.colors.primary.light,
    color: theme.colors.background.light,
    border: 'none',
    cursor: 'pointer',
    fontSize: '1rem',
    fontWeight: 600,
    boxShadow: theme.shadows.medium,
    transition: 'background-color 0.3s',
    textDecoration: 'none',
  };

  const hoverEffect = (
    e: React.MouseEvent<HTMLAnchorElement, MouseEvent>,
    color: string
  ) => {
    (e.target as HTMLAnchorElement).style.backgroundColor = color;
  };

  return (
    <div style={containerStyle}>
      <h1 style={{ fontSize: '2.5rem', fontWeight: 'bold', margin: '0.5em 0' }}>
        Welcome to Communest
      </h1>
      <p style={{ fontSize: '1rem', margin: '0.5em 0' }}>
        Discover a modern way to manage your property effectively and connect
        with your community.
      </p>
      <NavLink
        to="/signup"
        style={buttonStyle}
        onMouseOver={(e) => hoverEffect(e, theme.colors.primary.dark)}
        onMouseOut={(e) => hoverEffect(e, theme.colors.primary.light)}
      >
        Get Started Today
      </NavLink>
      <p>
        Already have an account?{' '}
        <NavLink to="/login" style={{ textDecoration: 'underline' }}>
          Log in
        </NavLink>
      </p>
    </div>
  );
};

export default WelcomePage;
