import Card from '../components/Cards/Card';
import Button from '../components/Buttons/Button';

const TasksPage = () => {
  return (
    <div>
      <Card title="Task 1">
        <p>Task description here...</p>
        <Button
          label="Complete Task"
          onClick={() => console.log('Task completed')}
        />
      </Card>
      {/* More tasks */}
    </div>
  );
};

export default TasksPage;
