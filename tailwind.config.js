/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: '#1976D2',
        secondary: '#26A69A',
      },
      backdropBlur: {
        '4': '4px'
      }
    },
  },
  plugins: [],
}
