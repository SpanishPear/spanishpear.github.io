module.exports = {
  darkMode: 'class',
  purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {
        extend: {
            colors: {
                'th-primary': '#fee2e2',
                'th-secondary': '#f0f0f0',
                'th-highlighting':'#34d399',
                'th-tabclicked': '#d19292',
                'landing-navbar': 'rgba(15, 23, 42, 0.85)',
                'landing-container-end': '#0b111f',
            }
        },
    },
    variants: {},
    plugins: [],
};
