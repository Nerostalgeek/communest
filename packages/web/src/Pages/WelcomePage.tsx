const WelcomePage = () => {
  return (
    <div className="container mx-auto px-4 py-10">
      <h1 className="text-4xl font-bold text-primary mb-8">Color Showcase</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {/* Color Sections */}
        {[
          {
            name: 'Primary',
            colorClass: 'bg-primary',
            textColor: 'text-textLight',
          },
          {
            name: 'Secondary',
            colorClass: 'bg-secondary',
            textColor: 'text-textLight',
          },
          {
            name: 'Accent',
            colorClass: 'bg-accent',
            textColor: 'text-gray-800',
          },
          {
            name: 'Light Background',
            colorClass: 'bg-background',
            textColor: 'text-gray-800',
          },
          {
            name: 'Dark Background',
            colorClass: 'bg-backgroundDark',
            textColor: 'text-textLight',
          },
        ].map(({ name, colorClass, textColor }) => (
          <div
            key={name}
            className={`${colorClass} p-4 rounded flex items-center justify-center`}
          >
            <p className={`${textColor} text-lg font-semibold`}>{name} Color</p>
          </div>
        ))}

        {/* Buttons Showcase */}
        <div className="col-span-full">
          <h2 className="text-2xl font-semibold text-secondary mb-4">
            Buttons
          </h2>
          <div className="flex space-x-4">
            <button className="bg-primary text-textLight px-4 py-2 rounded">
              Primary Button
            </button>
            <button className="bg-secondary text-textLight px-4 py-2 rounded">
              Secondary Button
            </button>
            <button className="bg-accent text-gray-800 px-4 py-2 rounded">
              Accent Button
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default WelcomePage;
