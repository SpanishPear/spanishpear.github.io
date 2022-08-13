module.exports = {
  darkMode: 'class',
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
    "../yew-markdown/src/lib.rs",
  ],
  theme: {
      extend: {
          colors: {
              'th-primary': '#fee2e2',
              'th-secondary': '#f0f0f0',
              'th-highlighting':'#34d399',
              'th-tabclicked': '#d19292',
              'landing-navbar': '#a530cf',
              'landing-container-end': '#381555',
          }
      },
      screens: {
        'xs': '350px',
        // => @media (min-width: 370px) { ... }
        
        'sm': '640px',
        // => @media (min-width: 640px) { ... }

        'md': '768px',
        // => @media (min-width: 768px) { ... }

        'lg': '1024px',
        // => @media (min-width: 1024px) { ... }

        'xl': '1280px',
        // => @media (min-width: 1280px) { ... }

        '2xl': '1536px',
        // => @media (min-width: 1536px) { ... } 
      },
  },
  variants: {},
  plugins: [],
};
