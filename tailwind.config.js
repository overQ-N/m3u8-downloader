/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        theme: {
          default: 'var(--sunzi-theme-color)',
          ...[10, 20, 30, 40, 50, 60, 70, 80].reduce((prev, curr, index) => ({
            ...prev,
            [`o-${index + 1}`]: `var(--sunzi-theme-color-o${index + 1})`
          }), {})
        }
      }
    },
  },
  plugins: [],
}

