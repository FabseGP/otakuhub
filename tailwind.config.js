/** @type {import('tailwindcss').Config} */
    module.exports = {
      content: {
        relative: true,
        files: ["*.html", "./src/**/*.rs"],
      },
      theme: {
        extend: {
          fontFamily: {
          }
        },
      },
	    plugins: [require("@tailwindcss/typography"), require('daisyui'),],

      daisyui: {
        themes: [
          "night",
          "light",
          "cupcake",
          "bumblebee",
          "emerald",
          "corporate",
          "synthwave",
          "retro",
          "cyberpunk",
          "valentine",
          "halloween",
          "garden",
          "forest",
          "aqua",
          "lofi",
          "pastel",
          "fantasy",
          "wireframe",
          "black",
          "luxury",
          "dracula",
          "cmyk",
          "autumn",
          "business",
          "acid",
          "lemonade",
          "dark",
          "coffee",
          "winter",
          "dim",
          "nord",
          "sunset",
        ],
      },
    }
    
