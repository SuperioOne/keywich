const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
    // 1. Apply the dark mode class setting:
    darkMode: 'class',
    content: [
        './src/**/*.{html,js,svelte,ts}',
        // 2. Append the path for the Skeleton NPM package and files:
        require('path').join(require.resolve(
            '@skeletonlabs/skeleton'),
            '../**/*.{html,js,svelte,ts}'
        )
    ],
    theme: {
        screens: {
            'xs': { max:'450px'},
            ...defaultTheme.screens,
          },
    },
    plugins: [
        require('@tailwindcss/forms'),
        // 3. Append the Skeleton plugin to the end of this list
        ...require('@skeletonlabs/skeleton/tailwind/skeleton.cjs')()
    ]
}