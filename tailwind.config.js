/** @type {import('tailwindcss').Config} */
import plugin from 'tailwindcss/plugin'
import daisyui from 'daisyui'
const pxToRem = (px) => ({
  [px / 4]: `${px / 16}rem`,
});

const pxToRemWithString = (px) => `${px / 16}rem`;

const evenArr = [
  0, 2, 3, 6, 10, 11, 12, 13, 14, 15, 18, 22, 26, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52,
  54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 75, 76, 78, 82, 84, 86, 88, 90, 94, 96, 98, 100, 102, 104,
  106, 108, 110, 112, 114, 118, 120, 122, 124, 132, 134, 135, 136, 138, 140, 144, 150, 152, 154,
  156, 158, 160, 168, 170, 172, 174, 176, 178, 180, 182, 188, 190, 198, 200, 208, 210, 212, 216,
  220, 222, 224, 228, 230, 232, 238, 240, 244, 246, 248, 250, 256, 260, 268, 272, 280, 284, 286,
  298, 300, 302, 304, 308, 310, 318, 320, 322, 324, 326, 328, 332, 336, 344, 350, 352, 360, 366,
  368, 372, 374, 375, 376, 380, 384, 386, 396, 400, 404, 408, 410, 416, 418, 422, 420, 424, 440,
  450, 452, 456, 460, 476, 480, 488, 500, 502, 504, 520, 532, 534, 552, 560, 564, 568, 600, 610,
  628, 630, 656, 658, 666, 680, 688, 690, 700, 720, 740, 744, 800, 840, 860, 892, 924, 944, 948,
  960, 1020, 1060, 1118, 1120, 1156, 1160, 1180, 1200, 1392, 1396, 1440, 1680, 1776, 1960,
];
const evenRemArray = evenArr.map(pxToRem);

export default {
  content: ["./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        gray: {
          0: '#ffffff',
          1: '#f9fafb',
          2: '#f3f4f6',
          3: '#e5e7eb',
          4: '#d1d5db',
          5: '#9ca3af',
          6: '#6b7280',
          7: '#111827',
          'alpha-12': 'rgba(17, 24, 39, 0.12)',
          'alpha-40': 'rgba(17, 24, 39, 0.40)',
          'alpha-75': 'rgba(17, 24, 39, 0.75)',
        },
        theme: {
          default: 'var(--theme-color)',
          ...[6, 10, 20, 30, 40, 50, 60, 70, 80].reduce((prev, curr) => ({
            ...prev,
            [`alpha-${curr}`]: `var(--theme-color-alpha-${curr})`
          }), {})
        }
      },
      width: Object.assign({}, ...evenRemArray),
      height: Object.assign({}, ...evenRemArray),
      padding: Object.assign(
        {
          full: '100%',
        },
        ...evenRemArray,
      ),
      margin: Object.assign(
        {
          full: '100%',
        },
        ...evenRemArray,
      ),
      maxWidth: Object.assign({}, ...evenRemArray),
    },
  },
  plugins: [daisyui, plugin(function ({ addBase }) {
    addBase({
      ".test": {
        "display": "flex"
      }
    })
  })],
  daisyui: {
    themes: ["bumblebee", "cupcake", "dark"],
  }
}

