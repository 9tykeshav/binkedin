import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        voodoo: {
          "50": "#fbf8fb",
          "100": "#f7f0f7",
          "200": "#ede0ee",
          "300": "#dfc6e1",
          "400": "#cba4ce",
          "500": "#b37fb6",
          "600": "#966099",
          "700": "#7d4e7d",
          "800": "#694269",
          "900": "#563955",
          "950": "#351d34",
        },
      },
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic":
          "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
    },
  },
  plugins: [],
};
export default config;
