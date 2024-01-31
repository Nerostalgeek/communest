const WelcomePage = () => {
  const colors = [
    { name: 'Primary', colorClass: 'bg-primary' },
    { name: 'Secondary', colorClass: 'bg-secondary' },
    { name: 'Accent', colorClass: 'bg-accent' },
    { name: 'Light Background', colorClass: 'bg-background-light' },
    { name: 'Dark Background', colorClass: 'bg-background-dark' },
    // Add more colors as needed
  ];

  return (
    <div className="container mx-auto px-4 py-10">
      <h1 className="text-4xl font-bold mb-8">Color Showcase</h1>

      {/* Color Blocks */}
      <div className="flex flex-wrap gap-4 justify-center items-center mb-8">
        {colors.map(({ name, colorClass }) => (
          <div
            key={name}
            className={`${colorClass} p-4 rounded-lg w-36 h-36 flex items-center justify-center`}
          >
            <p className="text-lg font-semibold">{name}</p>
          </div>
        ))}
      </div>

      {/* Buttons */}
      <div className="flex gap-4 mb-8">
        {colors.map(({ name, colorClass }) => (
          <button key={name} className={`${colorClass} py-2 px-4 rounded`}>
            {name} Button
          </button>
        ))}
      </div>

      {/* Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-4">
        {colors.map(({ name, colorClass }) => (
          <div key={name} className={`${colorClass} p-4 rounded-lg shadow-lg`}>
            <h2 className="text-lg font-semibold mb-2">{name} Card</h2>
            <p>
              This is a sample card with the {name.toLowerCase()} color theme.
            </p>
          </div>
        ))}
      </div>
      {/* Larger Square Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-8">
        {colors.map(({ name, colorClass }) => (
          <div key={name} className={`${colorClass} p-6 rounded-lg shadow-xl`}>
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
