// src/Components/LoginSignup/LoginSignup.tsx

import React, { useState } from 'react';
import './LoginSignup.css';
import RecaptchaScreen from '../Auth/ReCaptchaScreen'; // Adjusted import path
import { backendService } from '../../services/backendService'; // Adjusted path if necessary
//import logo from '../Assets/logo.svg'

import user_icon from '../Assets/person.png';
import email_icon from '../Assets/email.png';
import password_icon from '../Assets/password.png';

const LoginSignup = () => {
  const [isSignup, setIsSignup] = useState(true);
  const [showRecaptcha, setShowRecaptcha] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string>('');

  const toggleForm = () => {
    setIsSignup(!isSignup);
    setErrorMessage('');
  };

  const handleSignupOrLogin = async () => {
    setErrorMessage('');
    try {
      if (isSignup) {
        // Handle signup
        await handleSignup();
      } else {
        // Handle login
        await handleLogin();
      }
      setShowRecaptcha(true);
    } catch (error: any) {
      setErrorMessage(error.message);
    }
  };

  const handleSignup = async () => {
    // Implement signup logic here
    console.log('Handling signup...');
    // Example implementation
    const signedUp = await backendService.authenticateUser('test', 'password'); // Adjust parameters as necessary
    if (!signedUp) {
      throw new Error('Signup failed. Please try again.');
    }
    console.log('Signup successful');
  };

  const handleLogin = async () => {
    // Implement login logic here
    console.log('Handling login...');
    // Example implementation
    const loggedIn = await backendService.authenticateUser('test', 'password'); // Adjust parameters as necessary
    if (!loggedIn) {
      throw new Error('Invalid username or password');
    }
    console.log('Login successful');
  };

  return (
    <div className="login-signup-container">
      {!showRecaptcha ? (
        <div className="form-container">
          {/* <div className="logo-container">
            <img src={logo} alt="Logo" className="logo" />
          </div> */}
          <div className="header">
            <div className="text">{isSignup ? 'Sign up' : 'Login'}</div>
            <div className="underline"></div>
          </div>
          <div className="inputs">
            {isSignup && (
              <div className="input">
                <img src={user_icon} alt="User Icon" />
                <input type="text" placeholder="Nickname" />
              </div>
            )}
            <div className="input">
              <img src={email_icon} alt="Email Icon" />
              <input type="email" placeholder="Email" />
            </div>
            <div className="input">
              <img src={password_icon} alt="Password Icon" />
              <input type="password" placeholder="Password" />
            </div>
          </div>
          {!isSignup && (
            <div className="forgot-password">
              Lost Password? <span>Click Here!</span>
            </div>
          )}
          {errorMessage && <div className="error-message">{errorMessage}</div>}
          <div className="submit-container">
            <div className="submit" onClick={handleSignupOrLogin}>
              {isSignup ? 'Sign up' : 'Login'}
            </div>
          </div>
          <div className="toggle-container">
            <button onClick={toggleForm}>
              {isSignup ? 'Switch to Login' : 'Switch to Signup'}
            </button>
          </div>
        </div>
      ) : (
        <RecaptchaScreen />
      )}
    </div>
  );
};

export default LoginSignup;
