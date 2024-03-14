/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Nunito Sans', 'Inter', 'sans-serif'],
      }
    },
  },
  plugins: [],
}

