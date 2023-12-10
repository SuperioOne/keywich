import {is_null_or_empty} from "$lib";

/**
 * Formats a string by replacing placeholders with provided arguments.
 *
 * @param {string} format - The format string with placeholders.
 * @param {...any[]} args - The arguments to replace the placeholders.
 * @return {string} - The formatted string.
 */
export function format_string(format: string, ...args: any[]): string;
/**
 * Formats a string by replacing placeholders with named arguments.
 *
 * @param {string} format - The string with placeholders to be replaced.
 * @param {Record<string, any>} namedArgs - The object containing named arguments.
 * @returns {string} - The formatted string.
 */
export function format_string(format: string, namedArgs: Record<string, any>): string;
export function format_string(format: string, args: Record<string, any> | any[]): string {
  let params: Record<string, any>;

  if (Array.isArray(args)) {
    params = {};

    for (let i = 0; i < args.length; i++) {
      params[i.toString()] = args[i];
    }
  } else {
    params = args;
  }

  return formatter(format, params);
}

const CharMap = {
  OpenCurly: "{".charCodeAt(0),
  CloseCurly: "}".charCodeAt(0),
  Whitespace: " ".charCodeAt(0),
} as const

/**
 * Replaces placeholders in the given text with values from a lookup map.
 *
 * @param {string} text - The text to format.
 * @param {Record<string, string>} lookupMap - The lookup map containing placeholder-value pairs.
 * @return {string} The formatted text with replaced placeholders.
 */
function formatter(text: string, lookupMap: Record<string, string>) {
  if (is_null_or_empty(text)) return text;

  let slices: string[] = [];
  let templateIndex = -1;
  let sliceStart = 0;

  for (let i = 0; i < text.length; i++) {
    let charCode = text.codePointAt(i);

    if (charCode === CharMap.OpenCurly) {
      templateIndex = i;

    } else if (charCode === CharMap.CloseCurly && templateIndex > -1) {
      if (templateIndex + 1 === i) {
        templateIndex = -1;
        continue;
      }

      if (sliceStart < templateIndex) {
        slices.push(text.slice(sliceStart, templateIndex))
      }

      const key = text.slice(templateIndex + 1, i);
      slices.push(lookupMap[key] ?? "");
      templateIndex = -1;
      sliceStart = i + 1;

    } else if ((charCode === CharMap.Whitespace || charCode === CharMap.OpenCurly) && templateIndex > -1) {
      templateIndex = -1;
    }
  }

  if (sliceStart < text.length) {
    slices.push(text.slice(sliceStart))
  }

  return slices.join("");
}