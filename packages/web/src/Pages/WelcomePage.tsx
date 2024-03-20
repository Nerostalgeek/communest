import React from 'react';
import { theme } from '@communest/shared';
import Card from '../components/Cards/Card';
import Button from '../components/Buttons/Button';

const WelcomePage: React.FC = () => {
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
        className="text-center p-20 mb-10"
        style={{
          backgroundColor: theme.colors.primary.light,
          color: theme.colors.text.light,
        }}
      >
        <h1 className="text-5xl font-bold mb-6">Welcome to Home Manager</h1>
        <p>Everything you need to run your home smoothly, all in one place.</p>
      </section>

      {/* Grid of Feature Cards */}
      <section className="mb-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {features.map(({ name, description, icon }, index) => (
          <Card key={index} title={`${icon} ${name}`}>
            <p>{description}</p>
          </Card>
        ))}
      </section>

      {/* Call to Action */}
      <div className="text-center mb-10">
        <Button
          label="Get Started"
          onClick={() => console.log('Navigate to sign up')}
        />
      </div>

      {/* Testimonials or Tips Section */}
      <section
        className="p-10 rounded-lg"
        style={{
          backgroundColor: theme.colors.neutral.light,
          color: theme.colors.text.light,
        }}
      >
        <h2 className="text-2xl font-bold mb-4">Why Home Manager?</h2>
        <p>
          Discover how Home Manager can simplify your daily routines, from
          tracking tasks to managing expenses.
        </p>
      </section>
    </div>
  );
};

export default WelcomePage;
