import { useTheme } from '../App';

interface ContainerProps {
  children: React.ReactNode;
}

const Container: React.FC<ContainerProps> = ({ children }) => {
  const { darkMode, theme } = useTheme();

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

  return <div style={containerStyle}>{children}</div>;
};

export default Container;
