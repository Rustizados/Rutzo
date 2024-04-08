/** @type {import('tailwindcss').Config} */
export default {
  content: [
  "./src/**/*.{js,jsx,ts,tsx}",
  // Incluye otras rutas según sea necesario
  ],
  theme: {
    extend: {
      colors: {
        primary: '#7834CF',
        secondary: '#3DDC84',
        'light-blue': '#89C6FF',
        'dark-blue': '#3633D0',
        background: '#080614',
        'background-gradient': 'linear-gradient(to right, var(--background), #17113A)',
        gradient: 'linear-gradient(to right, var(--primary-color), var(--secondary-color))',
        'blue-gradient': 'linear-gradient(to right, var(--light-blue), var(--dark-blue))',
        acrylic: '#ffffff26',
        'green-acrylic': '#4DAC74',
        'red-acrylic': '#AF4F3D',
        'blue-acrylic': '#3633D0',
      },
    },
  },
  plugins: [],
}
