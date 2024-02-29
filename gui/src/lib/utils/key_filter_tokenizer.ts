/**
 * Represents a type of token.
 */
export type TokenType = {
  type: "username" | "domain" | "tag" | "term"
  value: string
}

/**
 * Tokenizes and filters the given text.
 *
 * @param text - The text to tokenize and filter.
 * @return - Array of token types.
 */
export function tokenize_filter_query(text: string): TokenType[] {
  const tokens: TokenType[] = [];
  const words = text.split(/\s/gi);
  const iterator = words[Symbol.iterator]();
  let current: IteratorResult<string, string> = iterator.next();

  while (!current.done) {
    tokens.push(resolve_token(current.value));
    current = iterator.next();
  }

  return tokens;
}

/**
 * Resolves type of input and returns the corresponding TokenType.
 *
 * @param input - The input string to be resolved.
 * @return - The resolved TokenType object.
 */
function resolve_token(input: string): TokenType {
  const first_semicolon = input.indexOf(":");

  if (first_semicolon > -1) {
    const tagName = input.slice(0, first_semicolon).toLowerCase();

    switch (tagName) {
      case "tag":
      case "username":
      case "domain":
        return {
          type: tagName,
          value: input.slice(first_semicolon + 1)
        };
      default:
        break;
    }
  }

  return {
    type: "term",
    value: input
  }
}
