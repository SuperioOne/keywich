import {Log} from "$lib/logger";

const WHITE_SPACE = /^\s$/;

/**
 * Represents a type of token.
 */
export type TokenType = {
  type: "username" | "domain" | "tag" | "term" | "space",
  value: string
}

/**
 * Tokenizes and filters the given text.
 *
 * @param text - The text to tokenize and filter.
 * @return - Array of token types.
 */
export function tokenize_filter_query(text: string): TokenType[] {
  const tokens = tokenize(text);
  return tokens.map(e => resolve_token_type(e));
}

/**
 * Resolves type of input and returns the corresponding TokenType.
 *
 * @param input - The input string to be resolved.
 * @return - The resolved TokenType object.
 */
function resolve_token_type(input: string): TokenType {
  const first_colon = input.indexOf(":");

  if (first_colon > -1) {
    const tagName = input.slice(0, first_colon).toLowerCase();

    switch (tagName) {
      case "tag":
      case "username":
      case "domain":
        return {type: tagName, value: input.slice(first_colon + 1)};
      default:
        break;
    }
  }

  if (WHITE_SPACE.test(input)) {
    return {type: "space", value: '\xa0'};
  } else {
    return {type: "term", value: input};
  }
}

function tokenize(text: string): string[] {
  const tokens: string[] = [];
  let select_start = -1;

  for (let i = 0; i < text.length; i++) {
    const char = text[i];

    if (WHITE_SPACE.test(char)) {
      if (select_start > -1) {
        tokens.push(text.slice(select_start, i));
      }

      tokens.push(char);
      select_start = -1;
    } else if (select_start < 0) {
      select_start = i;
    }
  }

  if (select_start > -1) {
    tokens.push(text.slice(select_start));
  }

  return tokens;
}
