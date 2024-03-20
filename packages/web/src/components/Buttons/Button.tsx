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
        backgroundColor: theme.colors.primary,
        color: theme.colors.text,
        padding: `${theme.spacing.sm}px ${theme.spacing.md}px`,
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
