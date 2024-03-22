import React from 'react';
import { useTheme } from '../App';

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

  const getPadding = (size: string) => {
    switch (size) {
      case 'small':
        return `${theme.spacing.xs}px ${theme.spacing.sm}px`;
      case 'medium':
        return `${theme.spacing.sm}px ${theme.spacing.md}px`;
      case 'large':
        return `${theme.spacing.md}px ${theme.spacing.lg}px`;
      default:
        return `${theme.spacing.sm}px ${theme.spacing.md}px`;
    }
  };

  const getFontSize = (size: string) => {
    switch (size) {
      case 'small':
        return theme.typography.fontSize.small;
      case 'medium':
        return theme.typography.fontSize.medium;
      case 'large':
        return theme.typography.fontSize.large;
      default:
        return theme.typography.fontSize.medium;
    }
  };

  const baseStyle = {
    padding: getPadding(size),
    fontSize: getFontSize(size),
    backgroundColor: darkMode
      ? theme.colors[variant].dark
      : theme.colors[variant].light,
    color: theme.colors.text.light, // Assuming your theme has a 'text' color scheme
    border: 'none',
    borderRadius: theme.borders.radius.default, // Assuming your theme has a 'borders' definition
    cursor: 'pointer',
    transition: 'background-color 0.3s', // Consider moving this to your theme if you want it customizable
  };

  return (
    <button style={baseStyle} {...props}>
      {children}
    </button>
  );
};

export default Button;
