import React from 'react';
import { useTheme } from '../../App';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'accent';
  size?: 'small' | 'medium' | 'large';
}

const Button: React.FC<ButtonProps> = ({
  children,
  variant = 'primary',
  size = 'medium',
  ...props
}) => {
  const { darkMode, theme } = useTheme();
  const baseStyle = {
    padding:
      size === 'small'
        ? '8px 12px'
        : size === 'medium'
          ? '10px 15px'
          : '12px 18px',
    fontSize:
      size === 'small' ? '0.875rem' : size === 'medium' ? '1rem' : '1.125rem',
    backgroundColor: darkMode
      ? theme.colors[variant].dark
      : theme.colors[variant].light,
    color: '#FFFFFF',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
    transition: 'background-color 0.3s',
  };

  return (
    <button style={baseStyle} {...props}>
      {children}
    </button>
  );
};

export default Button;
