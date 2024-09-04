import { useState } from 'react';
import Input from '../../components/Input'; 
import Button from '../../components/Button';
import Container from '../../components/Container'; 

const SignupPage: React.FC = () => {
  const [lastName, setLastName] = useState('');
  const [firstName, setFirstName] = useState('');
  const [email, setEmail] = useState('');
  const [phoneNumber, setPhoneNumber] = useState('');
  const [password, setPassword] = useState('');

  const handleSignup = async (event: React.FormEvent) => {
    event.preventDefault();
    console.log('Signup Attempt:', {
      lastName,
      firstName,
      email,
      phoneNumber,
      password,
    });
  };

  return (
    <Container>
      <form onSubmit={handleSignup} style={{ width: '300px' }}>
        <h1>Create Your Account</h1>
        <Input
          type="text"
          placeholder="Last Name"
          required
          value={lastName}
          onChange={(e) => setLastName(e.target.value)}
        />
        <Input
          type="text"
          placeholder="First Name"
          required
          value={firstName}
          onChange={(e) => setFirstName(e.target.value)}
        />
        <Input
          type="email"
          placeholder="Email"
          required
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <Input
          type="tel"
          placeholder="Phone Number (optional)"
          value={phoneNumber}
          onChange={(e) => setPhoneNumber(e.target.value)}
        />
        <Input
          type="password"
          placeholder="Password"
          required
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <Button type="submit" variant="primary">
          Sign Up
        </Button>
      </form>
    </Container>
  );
};

export default SignupPage;
