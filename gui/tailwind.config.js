import defaultTheme from "tailwindcss/defaultTheme";
import tailwindFormPlugin from "@tailwindcss/forms"
import {skeleton} from "@skeletonlabs/tw-plugin";
import {keywich_theme} from "./themes/keywich_theme";

// @ts-check
import {join} from 'path';

/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: [
    './src/**/*.{html,js,svelte,ts}',
    join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')
  ],
  theme: {
    defaultTheme: defaultTheme,
    extend: {},
  },
  plugins: [
    tailwindFormPlugin,
    skeleton({
      themes: {
        custom: [
          keywich_theme
        ],
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