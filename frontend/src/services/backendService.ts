// src/services/backendService.ts

export const backendService = {
    authenticateUser(username: string, password: string): boolean {
      // Simulate authentication logic
      if (username === 'test' && password === 'password') {
        return true; // Return true if authentication is successful
      } else {
        return false; // Return false if authentication fails
      }
    }
  };
  