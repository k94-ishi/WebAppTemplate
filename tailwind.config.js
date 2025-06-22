import typography from '@tailwindcss/typography'
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      backgroundImage: {
        'hero-pattern': "url('assets/pexels-photo-1450360.jpeg')",
      },
      fontFamily: {
        sans: ['"Noto Sans JP"', 'sans-serif'],
      },
      colors: {
        primary: '#4f46e5',
        secondary: '#6b7280',
      },
    },
  },
  plugins: [typography],
}
