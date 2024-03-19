import React from 'react';
import Card from '../components/Cards/Card';
import Button from '../components/Buttons/Button';
import { theme } from '@communest/shared';

const WelcomePage: React.FC = () => {
  // Define sections/features of house management to highlight
  const features = [
    {
      name: 'Tasks',
      description: 'Manage your daily tasks efficiently.',
      icon: '📋',
    },
    {
      name: 'Events',
      description: 'Keep track of upcoming family events.',
      icon: '📅',
    },
    {
      name: 'Chores',
      description: 'Distribute household chores fairly.',
      icon: '🧹',
    },
    {
      name: 'Food',
      description: 'Plan your meals and grocery shopping.',
      icon: '🍽️',
    },
    {
      name: 'Budget',
      description: 'Monitor your household expenses.',
      icon: '💰',
    },
  ];

  return (
    <div className="container mx-auto px-4 py-10">
      {/* Full-width Banner Section */}
      <section
        style={{
          backgroundColor: theme.colors.primary.DEFAULT,
          color: theme.colors.text.DEFAULT,
          textAlign: 'center',
          padding: '80px 20px',
          marginBottom: '40px',
        }}
      >
        <h1
          style={{ fontSize: '48px', fontWeight: 'bold', marginBottom: '24px' }}
        >
          Welcome to Home Manager
        </h1>
        <p>Everything you need to run your home smoothly, all in one place.</p>
      </section>

      {/* Grid of Feature Cards */}
      <section
        style={{
          marginBottom: '40px',
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))',
          gap: '16px',
        }}
      >
        {features.map(({ name, description, icon }, index) => (
          <Card key={index} title={`${icon} ${name}`}>
            <p>{description}</p>
          </Card>
        ))}
      </section>

      {/* Call to Action */}
      <div style={{ textAlign: 'center', marginBottom: '40px' }}>
        <Button
          label="Get Started"
          onClick={() => console.log('Navigate to sign up')}
        />
      </div>

      {/* Testimonials or Tips Section */}
      <section
        style={{
          backgroundColor: theme.colors.neutral.DEFAULT,
          color: theme.colors.text.DEFAULT,
          padding: '40px',
          borderRadius: '8px',
        }}
      >
        <h2
          style={{ fontSize: '24px', fontWeight: 'bold', marginBottom: '16px' }}
        >
          Why Home Manager?
        </h2>
        <p>
          Discover how Home Manager can simplify your daily routines, from
          tracking tasks to managing expenses.
        </p>
      </section>
    </div>
  );
};

export default WelcomePage;
