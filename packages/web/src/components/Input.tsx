import { useTheme } from '../App';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {}

const Input: React.FC<InputProps> = (props) => {
  const { darkMode, theme } = useTheme();

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

  return <input style={{ ...inputStyle, ...props.style }} {...props} />;
};

export default Input;
