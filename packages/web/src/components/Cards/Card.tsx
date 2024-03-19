import React from 'react';
import { theme } from '@communest/shared';

interface CardProps {
  title: string;
  children: React.ReactNode;
}

const Card: React.FC<CardProps> = ({ title, children }) => {
  return (
    <div
      style={{
        backgroundColor: theme.colors.background.DEFAULT,
        color: theme.colors.text.DEFAULT,
        padding: theme.spacing.medium,
        borderRadius: '8px',
        boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
      }}
    >
      <h3>{title}</h3>
      {children}
    </div>
  );
};

export default Card;
