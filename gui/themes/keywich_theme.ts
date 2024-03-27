import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const keywich_theme: CustomThemeConfig = {
  name: "default",
  properties: {
    // =~= Theme Properties =~=
    "--theme-font-family-base": `system-ui`,
    "--theme-font-family-heading": `system-ui`,
    "--theme-font-color-base": "0 0 0",
    "--theme-font-color-dark": "255 255 255",
    "--theme-rounded-base": "16px",
    "--theme-rounded-container": "12px",
    "--theme-border-base": "2px",
    // =~= Theme On-X Colors =~=
    "--on-primary": "0 0 0",
    "--on-secondary": "255 255 255",
    "--on-tertiary": "0 0 0",
    "--on-success": "0 0 0",
    "--on-warning": "0 0 0",
    "--on-error": "255 255 255",
    "--on-surface": "255 255 255",
    // =~= Theme Colors  =~=
    // primary | #8ff0a4
    "--color-primary-50": "238 253 241", // #eefdf1
    "--color-primary-100": "233 252 237", // #e9fced
    "--color-primary-200": "227 251 232", // #e3fbe8
    "--color-primary-300": "210 249 219", // #d2f9db
    "--color-primary-400": "177 245 191", // #b1f5bf
    "--color-primary-500": "143 240 164", // #8ff0a4
    "--color-primary-600": "129 216 148", // #81d894
    "--color-primary-700": "107 180 123", // #6bb47b
    "--color-primary-800": "86 144 98", // #569062
    "--color-primary-900": "70 118 80", // #467650
    // secondary | #1a5fb4
    "--color-secondary-50": "221 231 244", // #dde7f4
    "--color-secondary-100": "209 223 240", // #d1dff0
    "--color-secondary-200": "198 215 236", // #c6d7ec
    "--color-secondary-300": "163 191 225", // #a3bfe1
    "--color-secondary-400": "95 143 203", // #5f8fcb
    "--color-secondary-500": "26 95 180", // #1a5fb4
    "--color-secondary-600": "23 86 162", // #1756a2
    "--color-secondary-700": "20 71 135", // #144787
    "--color-secondary-800": "16 57 108", // #10396c
    "--color-secondary-900": "13 47 88", // #0d2f58
    // tertiary | #0EA5E9
    "--color-tertiary-50": "219 242 252", // #dbf2fc
    "--color-tertiary-100": "207 237 251", // #cfedfb
    "--color-tertiary-200": "195 233 250", // #c3e9fa
    "--color-tertiary-300": "159 219 246", // #9fdbf6
    "--color-tertiary-400": "86 192 240", // #56c0f0
    "--color-tertiary-500": "14 165 233", // #0EA5E9
    "--color-tertiary-600": "13 149 210", // #0d95d2
    "--color-tertiary-700": "11 124 175", // #0b7caf
    "--color-tertiary-800": "8 99 140", // #08638c
    "--color-tertiary-900": "7 81 114", // #075172
    // success | #00db60
    "--color-success-50": "217 250 231", // #d9fae7
    "--color-success-100": "204 248 223", // #ccf8df
    "--color-success-200": "191 246 215", // #bff6d7
    "--color-success-300": "153 241 191", // #99f1bf
    "--color-success-400": "77 230 144", // #4de690
    "--color-success-500": "0 219 96", // #00db60
    "--color-success-600": "0 197 86", // #00c556
    "--color-success-700": "0 164 72", // #00a448
    "--color-success-800": "0 131 58", // #00833a
    "--color-success-900": "0 107 47", // #006b2f
    // warning | #e5a50a
    "--color-warning-50": "251 242 218", // #fbf2da
    "--color-warning-100": "250 237 206", // #faedce
    "--color-warning-200": "249 233 194", // #f9e9c2
    "--color-warning-300": "245 219 157", // #f5db9d
    "--color-warning-400": "237 192 84", // #edc054
    "--color-warning-500": "229 165 10", // #e5a50a
    "--color-warning-600": "206 149 9", // #ce9509
    "--color-warning-700": "172 124 8", // #ac7c08
    "--color-warning-800": "137 99 6", // #896306
    "--color-warning-900": "112 81 5", // #705105
    // error | #a51d2d
    "--color-error-50": "242 221 224", // #f2dde0
    "--color-error-100": "237 210 213", // #edd2d5
    "--color-error-200": "233 199 203", // #e9c7cb
    "--color-error-300": "219 165 171", // #dba5ab
    "--color-error-400": "192 97 108", // #c0616c
    "--color-error-500": "165 29 45", // #a51d2d
    "--color-error-600": "149 26 41", // #951a29
    "--color-error-700": "124 22 34", // #7c1622
    "--color-error-800": "99 17 27", // #63111b
    "--color-error-900": "81 14 22", // #510e16
    // surface | #272b33
    "--color-surface-50": "223 223 224", // #dfdfe0
    "--color-surface-100": "212 213 214", // #d4d5d6
    "--color-surface-200": "201 202 204", // #c9cacc
    "--color-surface-300": "169 170 173", // #a9aaad
    "--color-surface-400": "104 107 112", // #686b70
    "--color-surface-500": "39 43 51", // #272b33
    "--color-surface-600": "35 39 46", // #23272e
    "--color-surface-700": "29 32 38", // #1d2026
    "--color-surface-800": "23 26 31", // #171a1f
    "--color-surface-900": "19 21 25", // #131519
  },
};

