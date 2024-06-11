import React, { useEffect, useRef } from 'react';
import ReCAPTCHA from 'react-google-recaptcha';
import './ReCaptchaScreen.css'; // Zaimportuj plik CSS do stylizacji ekranu

const ReCaptchaScreen: React.FC = () => {
  const recaptchaRef = useRef<ReCAPTCHA>(null);

  useEffect(() => {
    const onLoadRecaptcha = () => {
      // Możesz dodać logikę, która ma być wykonana po załadowaniu reCAPTCHA
      console.log('reCAPTCHA loaded');
    };

    window.addEventListener('load', onLoadRecaptcha);

    return () => {
      window.removeEventListener('load', onLoadRecaptcha);
    };
  }, []);

  const handleVerifyRecaptcha = (token: string | null) => {
    // Tutaj możesz obsłużyć token reCAPTCHA (np. wysłać go do serwera)
    console.log('reCAPTCHA token:', token);
  };

  return (
    <div className="recaptcha-screen">
      {/* Zawartość ekranu reCAPTCHA */}
      <div className="recaptcha-container">
        {/* Komponent reCAPTCHA */}
        <ReCAPTCHA 
          ref={recaptchaRef}
          sitekey="YOUR_SITE_KEY" // Zmień na prawdziwy klucz swojej witryny
          onChange={handleVerifyRecaptcha}
        />
        {/* Możesz dostosować HTML i style w zależności od Twoich potrzeb */}
        <p>Proszę udowodnić, że jesteś człowiekiem.</p>
      </div>
    </div>
  );
};

export default ReCaptchaScreen;
