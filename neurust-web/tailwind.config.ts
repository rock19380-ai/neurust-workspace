import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx}",
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
    "./src/**/*.{js,ts,jsx,tsx}"
  ],
  theme: {
    extend: {
      colors: {
        background: "#050505",
        foreground: "#FAFAFA",
        primary: "#FF7E5F",
        secondary: "#00F2EA"
      }
    }
  },
  plugins: []
};

export default config;
