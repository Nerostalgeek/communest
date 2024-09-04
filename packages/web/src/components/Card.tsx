import React from 'react';
import { useTheme } from '../App';
interface CardProps {
  title: string;
  content: string;
  imageUrl?: string;
}

const Card: React.FC<CardProps> = ({ title, content, imageUrl }) => {
  const { darkMode } = useTheme();


  const cardStyle: React.CSSProperties = {
    backgroundColor: darkMode ? '#333333' : '#FFFFFF',
    color: darkMode ? '#E0E0E0' : '#333333',
    padding: '20px',
    borderRadius: '8px',
    boxShadow: '0 4px 6px rgba(0,0,0,0.1)',
    transition: 'all 0.3s ease',
  };

  const imageStyle: React.CSSProperties = {
    width: '100%',
    height: 'auto',
    objectFit: 'cover',
    borderRadius: '8px',
  };

  return (
    <div style={cardStyle}>
      {imageUrl && <img src={imageUrl} alt={title} style={imageStyle} />}
      <h2>{title}</h2>
      <p>{content}</p>
    </div>
  );
};

export default Card;
