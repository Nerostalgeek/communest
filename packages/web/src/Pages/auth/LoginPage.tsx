import { useState } from 'react';

import Input from '../../components/Input';
import Button from '../../components/Button';
import Container from '../../components/Container';
import { login } from '@communest/shared';
const LoginPage: React.FC = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = async (event: React.FormEvent) => {
    event.preventDefault();
    await login({ email, password });
  };

  return (
    <Container>
      <form
        onSubmit={handleLogin}
        style={{
          width: '300px',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
        }}
      >
        <h1>Login</h1>
        <Input
          type="email"
          placeholder="Email"
          required
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <Input
          type="password"
          placeholder="Password"
          required
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <Button type="submit" variant="primary">
          Log In
        </Button>
      </form>
    </Container>
  );
};

export default LoginPage;
