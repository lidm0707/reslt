/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{html,js,rs}",
    "./tailwind-safelist.txt"],
  theme: {
    extend: {},
  },
  plugins: [],
};
