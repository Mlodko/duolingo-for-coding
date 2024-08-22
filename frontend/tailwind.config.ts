import { type Config } from "tailwindcss";

export default {
  content: ["./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {},
    colors: {
      'dark-purple': '#2c0170',
      'darker-purple': '#0c002e',  
      'pink-ish': '#990299',
      'white': '#FFFFFF',
      "black": '#000000',
    }
  },
  plugins: [],
} satisfies Config;
