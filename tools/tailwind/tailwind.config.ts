/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./res/templates/**/*.html",
    "./res/templates/*.html",
    "./src/**/*.rs",
    "./src/*.rs",
  ],
  theme: {
    extend: {
      animation: {
        fadeIn: ".1s ease-out forwards fadeIn",
        fadeOut: ".1s ease-out forwards reverse fadeIn",
        slideInRight: ".8s ease-out forwards slideInRight",
        slideOutLeft: ".2s ease-in forwards slideOutLeft",
      },
      keyframes: {
        fadeIn: {
          from: { opacity: "0", "pointer-events": "none" },
        },
        slideInRight: {
          from: {
            transform: "translateX(20rem)",
            opacity: "0",
            "pointer-events": "none",
          },
        },
        slideOutLeft: {
          to: {
            transform: "translateX(-20rem)",
            opacity: "0",
            "pointer-events": "none",
          },
        },
      },
    },
  },
  plugins: [],
};
