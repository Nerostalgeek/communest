import React from 'react';
import { theme } from '@communest/shared';

interface ButtonProps {
  label: string;
  onClick: () => void;
}

const Button: React.FC<ButtonProps> = ({ label, onClick }) => {
  return (
    <button
      onClick={onClick}
      style={{
        backgroundColor: theme.colors.primary.DEFAULT,
        color: theme.colors.text.DEFAULT,
        padding: `${theme.spacing.small}px ${theme.spacing.medium}px`,
        border: 'none',
        borderRadius: '4px',
        cursor: 'pointer',
      }}
    >
      {label}
    </button>
  );
};

export default Button;
