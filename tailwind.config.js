/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: ["./api/templates/**/*.{html,js}"],
  theme: {
    fontFamily: {
      sans: ["Outfit", "sans-serif"],
      mono: [
        "ui-monospace",
        "SFMono-Regular",
        "Menlo",
        "Monaco",
        "Consolas",
        "Liberation Mono",
        "Courier New",
        "monospace",
      ],
    },
    fontSize: {
      xs: "0.9rem",
      sm: "1rem",
      base: ["1.25rem", { lineHeight: "1.5rem" }],
      lg: ["1.5rem", { lineHeight: "1.8rem" }],
      xl: ["2rem", { lineHeight: "2.2rem" }],
      "2xl": ["3.2rem", { lineHeight: "3.4rem" }],
      "3xl": ["3.8rem", { lineHeight: "4rem" }],
      "4xl": ["4.5rem", { lineHeight: "4.7rem" }],
      "5xl": ["5.5rem", { lineHeight: "5.7rem" }],
      "6xl": ["6.5rem", { lineHeight: "6.7rem" }],
      "7xl": ["7.5rem", { lineHeight: "7.7rem" }],
      "8xl": ["8.5rem", { lineHeight: "8.7rem" }],
      "9xl": ["9.5rem", { lineHeight: "9.7rem" }],
    },
    extend: {
      colors: {
        black: "#1f1f1f",
        white: "#fefcd9",
      },
      gridTemplateColumns: {
        header: "100px repeat(5, 1fr)",
      },
      gridTemplateRows: {
        main: "min-content 1fr min-content",
      },
      boxShadow: {
        "flat-green": "10px 10px #0ba750",
      },
      transitionProperty: {
        height: "height",
      },
    },
  },
  plugins: [],
};
