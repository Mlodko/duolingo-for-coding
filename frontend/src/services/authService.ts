// src/services/authService.ts

export const fakeAuthService = {
    async signup(nickname: string, email: string, password: string): Promise<void> {
      // Simulate signup process
      if (email === 'test@example.com' && password === 'password') {
        throw new Error('Email already exists'); // Simulate an error for existing user
      }
      // Simulate success
      console.log(`User signed up: ${nickname}, ${email}, ${password}`);
    },
  
    async login(email: string, password: string): Promise<void> {
      // Simulate login process
      if (email !== 'test@example.com' || password !== 'password') {
        throw new Error('Invalid username or password');
      }
      // Simulate success
      console.log(`User logged in: ${email}, ${password}`);
    },
  };
  