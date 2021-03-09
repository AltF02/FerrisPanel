/* eslint-disable global-require,import/no-extraneous-dependencies */

module.exports = {
  style: {
    postcss: {
      plugins: [
        require('tailwindcss'),
        require('autoprefixer'),
      ],
    },
  },
  eslint: {
    enable: true,
    mode: 'extends',
  },
};
