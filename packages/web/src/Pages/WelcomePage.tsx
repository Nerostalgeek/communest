import React from 'react';

const WelcomePage: React.FC = () => {
  // Sample colors array for demonstration purposes
  const colors = [
    { name: 'Primary', colorClass: 'bg-primary-light dark:bg-primary-dark' },
    {
      name: 'Secondary',
      colorClass: 'bg-secondary-light dark:bg-secondary-dark',
    },
    { name: 'Accent', colorClass: 'bg-accent-light dark:bg-accent-dark' },
    // Add more color classes as needed
  ];

  return (
    <div className="container mx-auto px-4 py-10">
      {/* Full-width Banner Section */}
      <section
        className={`${colors[0].colorClass} text-textPrimary-light dark:text-textPrimary-dark text-center p-20 mb-10`}
      >
        <h1 className="text-6xl font-bold mb-6">Full-Width Banner</h1>
        <p>Expansive section with a striking color and large text.</p>
      </section>

      {/* Grid of Feature Cards */}
      <section className="mb-10 grid grid-cols-1 md:grid-cols-3 gap-4">
        {colors.map(({ name, colorClass }, index) => (
          <div key={index} className={`${colorClass} p-6 rounded-lg shadow-md`}>
            <h3 className="text-2xl font-semibold mb-3">{name}</h3>
            <p>Details of the feature in a medium-sized card.</p>
          </div>
        ))}
      </section>

      {/* Tall Image or Content Section */}
      <section className="bg-background-dark text-textLight p-10 mb-10">
        <div className="h-96 flex items-center justify-center">
          <p>
            This tall section is perfect for showcasing an image or elongated
            content.
          </p>
        </div>
      </section>

      {/* Horizontal Scroll Section for Smaller Items */}
      <section className="mb-10 overflow-x-auto whitespace-nowrap p-4 bg-background-light text-textDark">
        <div className="inline-block mr-4">
          <p>Small item 1</p>
        </div>
        <div className="inline-block mr-4">
          <p>Small item 2</p>
        </div>
        {/* Additional small items */}
      </section>

      {/* Wide Content Display */}
      <section className="bg-accent text-gray-800 p-10 mb-10">
        <div className="flex justify-between">
          <p>
            A wide content area for showcasing landscape images, wide tables, or
            other horizontal elements.
          </p>
        </div>
      </section>

      {/* Color Blocks */}

      <div className="flex flex-wrap gap-4 justify-center items-center mb-8">
        {colors.map(({ name, colorClass }, index) => (
          <div
            key={index}
            className={`${colorClass} p-4 rounded-lg w-36 h-36 flex items-center justify-center`}
          >
            <p className="text-lg font-semibold">{name}</p>
          </div>
        ))}
      </div>

      {/* Buttons */}
      <div className="flex gap-4 mb-8">
        {colors.map(({ name, colorClass }, index) => (
          <button key={index} className={`${colorClass} py-2 px-4 rounded`}>
            {name} Button
          </button>
        ))}
      </div>

      {/* Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-4">
        {colors.map(({ name, colorClass }, index) => (
          <div key={index} className={`${colorClass} p-4 rounded-lg shadow-lg`}>
            <h2 className="text-lg font-semibold mb-2">{name} Card</h2>
            <p>
              This is a sample card with the {name.toLowerCase()} color theme.
            </p>
          </div>
        ))}
      </div>

      {/* Larger Square Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-8">
        {colors.map(({ name, colorClass }, index) => (
          <div key={index} className={`${colorClass} p-6 rounded-lg shadow-xl`}>
            <h2 className="text-2xl font-bold mb-4">{name} Large Card</h2>
            <p className="mb-4">
              This is a larger card showcasing the {name.toLowerCase()} color.
              It provides more space to display content and illustrate how the
              color behaves in bigger UI elements.
            </p>
            <button className="py-2 px-4 border border-transparent rounded hover:border-gray-200">
              Learn More
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};

export default WelcomePage;
