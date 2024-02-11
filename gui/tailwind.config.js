import defaultTheme from "tailwindcss/defaultTheme";
import tailwindFormPlugin from "@tailwindcss/forms"
import {skeleton} from "@skeletonlabs/tw-plugin";

// @ts-check
import {join} from 'path';

/** @type {import('tailwindcss').Config} */
export default {
  // 2. Opt for dark mode to be handled via the class method
  darkMode: 'class',
  content: [
    './src/**/*.{html,js,svelte,ts}',
    // 3. Append the path to the Skeleton package
    join(require.resolve(
        '@skeletonlabs/skeleton'),
      '../**/*.{html,js,svelte,ts}'
    )
  ],
  theme: {
    defaultTheme: defaultTheme,
    extend: {},
  },
  plugins: [
    tailwindFormPlugin,
    skeleton({
      themes: {
        preset: [
          "crimson",
          "skeleton",
          "hamlindigo",
          "wintry",
          "rocket",
          "vintage",
          "modern",
          "seafoam",
          "gold-nouveau",
          "sahara"
        ],
      }
    })
  ]
}