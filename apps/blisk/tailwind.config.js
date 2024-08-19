import defaultTheme from "tailwindcss/defaultTheme";

/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["class", "[data-theme='dark']"],
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      transitionProperty: {
        "colors-opacity": `${defaultTheme.transitionProperty.colors}, opacity`,
      },
      fontFamily: {
        sans: "'Geist', Arial, sans-serif",
      },
      keyframes: {
        "progress-ring": {
          "100%": { transform: "rotate(360deg)" },
        },
        "progress-ring-s": {
          "0%": {
            "stroke-dasharray": "1, 200",
            "stroke-dashoffset": "0",
          },
          "66%": {
            "stroke-dasharray": "89, 200",
            "stroke-dashoffset": "-40px",
          },
          "100%": {
            "stroke-dasharray": "89, 200",
            "stroke-dashoffset": "-70px",
          },
        },
        "progress-ring-m": {
          "0%": {
            "stroke-dasharray": "1, 200",
            "stroke-dashoffset": "0",
          },
          "66%": {
            "stroke-dasharray": "89, 200",
            "stroke-dashoffset": "-35px",
          },
          "100%": {
            "stroke-dasharray": "89, 200",
            "stroke-dashoffset": "-100px",
          },
        },
        "progress-ring-l": {
          "0%": {
            "stroke-dasharray": "1, 200",
            "stroke-dashoffset": "0",
          },
          "66%": {
            "stroke-dasharray": "89, 200",
            "stroke-dashoffset": "-100px",
          },
          "100%": {
            "stroke-dasharray": "89, 200",
            "stroke-dashoffset": "-200px",
          },
        },
        fly: {
          from: {
            opacity: "0",
            transform: "translateY(var(--fly-translate-y, -1rem))",
          },
        },
        "thumbs-up": {
          "0%": {
            transform: "translateX(0px) translateY(0px) rotate(0deg) scale(1)",
          },
          "25%": {
            transform: "translateX(3px) translateY(0.5px) rotate(15deg) scale(0.9)",
          },
          "60%": {
            transform: "translateX(-4px) translateY(-1px) rotate(-15deg) scale(1.2)",
          },
          "75%": {
            transform: "translateX(3px) translateY(0.5px) rotate(15deg) scale(0.9)",
          },
        },
        heart: {
          "0%, 80%": {
            transform: "translateX(0px) translateY(0px) rotate(0deg) scale(1)",
          },
          "15%": {
            transform: "translateX(4px) translateY(4px) rotate(20deg) scale(.65)",
          },
          "35%": {
            transform: "translateX(-6px) translateY(1px) rotate(-20deg) scale(1.3)",
          },
          "50%": {
            transform: "translateX(0.5px) translateY(-4px) rotate(10deg) scale(1.25)",
          },
          "70%": {
            transform: "translateX(0.5px) translateY(1.5px) rotate(0deg) scale(.9)",
          },
        },
        "laughing-eyes": {
          "0%": {
            transform: "translateX(-2.25px) translateY(1px) rotate(-10deg) scale(1.07)",
          },
          "12%": {
            transform: "translateX(-1.75px) translateY(2px) rotate(-7deg) scale(1.05)",
          },
          "17%": {
            transform: "translateX(-1px) translateY(0.5px) rotate(-4deg) scale(1.05)",
          },
          "24%": {
            transform: "translateX(-0.5px) translateY(1px) rotate(-1deg) scale(1.05)",
          },
          "32%": {
            transform: "translateX(0.25px) translateY(-0.5px) rotate(2deg) scale(1.05)",
          },
          "40%": {
            transform: "translateX(0.75px) translateY(0.5px) rotate(5deg) scale(1.05)",
          },
          "47%": {
            transform: "translateX(1px) translateY(-0.5px) rotate(7deg) scale(1.05)",
          },
          "57%": {
            transform: "translateX(1px) translateY(2.5px) rotate(7deg) scale(1.05)",
          },
          "64%": {
            transform: "translateX(0.5px) translateY(1px) rotate(2deg) scale(1.05)",
          },
          "72%": {
            transform: "translateX(0px) translateY(2px) rotate(1deg) scale(1.05)",
          },
          "80%": {
            transform: "translateX(-0.5px) translateY(1.5px) rotate(-2deg) scale(1.05)",
          },
          "88%": {
            transform: "translateX(-1.25px) translateY(3px) rotate(-5deg) scale(1.05)",
          },
          "100%": {
            transform: "translateX(-2.25px) translateY(1px) rotate(-10deg) scale(1.07)",
          },
        },
        "laughing-mouth": {
          "0%": {
            transform: "translateX(-2px) translateY(1.5px) rotate(-10deg) scale(1.03)",
          },
          "12%": {
            transform: "translateX(-1.25px) translateY(4px) rotate(-7deg) scaleX(1.0) scaleY(0.8)",
          },
          "17%": {
            transform: "translateX(-0.75px) translateY(1px) rotate(-4deg) scale(1.02)",
          },
          "24%": {
            transform: "translateX(-0.25px) translateY(3px) rotate(0deg) scaleX(1.0) scaleY(0.8)",
          },
          "32%": {
            transform: "translateX(0.5px) translateY(0px) rotate(3deg) scale(1.02)",
          },
          "40%": {
            transform: "translateX(0.5px) translateY(2.5px) rotate(4deg) scaleY(0.8)",
          },
          "47%": {
            transform: "translateX(0.75px) translateY(0px) rotate(5deg) scale(1.02)",
          },
          "57%": {
            transform: "translateX(2px) translateY(6px) rotate(5deg) scaleX(0.8) scaleY(0.6)",
          },
          "64%": {
            transform: "translateX(0.5px) translateY(2.5px) rotate(2deg) scaleX(1.0) scaleY(0.85)",
          },
          "72%": {
            transform: "translateX(1px) translateY(4px) rotate(1deg) scaleX(0.9) scaleY(0.75)",
          },
          "80%": {
            transform: "translateX(0px) translateY(3.5px) rotate(-2deg) scaleX(1.0) scaleY(0.8)",
          },
          "88%": {
            transform: "translateX(0px) translateY(5px) rotate(-5deg) scaleX(0.9) scaleY(0.75)",
          },
          "100%": {
            transform: "translateX(-2px) translateY(1.5px) rotate(-10deg) scale(1.03)",
          },
        },
      },
      animation: {
        fly: "fly 100ms ease-out",
        like: "thumbs-up 1.5s ease-out 0.5s infinite",
        love: "heart 1.5s ease-out 0.5s infinite",
        "haha-eyes": "laughing-eyes 2.5s ease-out 0s infinite",
        "haha-mouth": "laughing-mouth 2.5s ease-out 0s infinite",
        ring: "progress-ring 0.9s linear infinite",
        "ring-sm": "progress-ring-s 2.4s linear infinite",
        "ring-md": "progress-ring-m 2.4s linear infinite",
        "ring-lg": "progress-ring-l 2.4s linear infinite",
      },
      backgroundImage: {
        wood: "url('/wood-pattern.png')",
        "wood-hor": "url('/wood-pattern-hor.png')",
        "dark-wood": "url('/dark-wood-pattern.png')",
        "dark-wood-hor": "url('/dark-wood-pattern-hor.png')",
      },
      colors: {
        wood: {
          50: "#fbf6ef",
          100: "#f7eedf",
          200: "#efdcbf",
          300: "#e8cb9f",
          400: "#e0b97f",
          500: "#d8a85f",
          550: "#ad864c",
          600: "#826539",
          650: "#856940",
          700: "#6b5330",
          800: "#3a2316",
          900: "#301a0e",
          915: "#23150d",
          925: "#1d110b",
          950: "#170e09",
          1000: "#110a06",
          1050: "#0b0704",
        },
        error: {
          light: "#a51a12",
          dark: "#f87171",
          hover: {
            light: "#8e1610",
            dark: "#f98b8b",
          },
          disabled: {
            light: "#73120d",
            dark: "#f99a9a",
          },
        },
        accent: {
          light: "#15224c",
          dark: "#7dd3fc",
        },
        border: {
          light: "#e0b97f",
          dark: "#23150d",
        },
        yellow: {
          75: "#FFF4CE",
          150: "#EFE8C1",
          925: "#665330",
          1000: "#433519",
        },
        neutral: {
          150: "#eeeeee",
          250: "#dddddd",
          915: "#111111",
          925: "#0d0d0d",
        },
      },
    },
  },
  plugins: [],
};
