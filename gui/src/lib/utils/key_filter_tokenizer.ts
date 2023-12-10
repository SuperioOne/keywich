export type TokenTypeName = "username" | "domain" | "tag" | "term";

/**
 * Represents a type of token.
 */
export type TokenType = {
  type: "username" | "domain" | "tag" | "term"
  value: string
}

/**
 * Represents the possible types of tags.
 */
export type TagType = "username" | "domain" | "tag";

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
    switch (current.value) {
      case "":
        break;
      case "tag:":
        tokens.push(resolve_tag_token(iterator, "tag"));
        break;
      case "username:":
        tokens.push(resolve_tag_token(iterator, "username"));
        break;
      case "domain:":
        tokens.push(resolve_tag_token(iterator, "domain"));
        break;
      default: {
        tokens.push(resolve_unknown_type(current.value));
        break;
      }
    }

    current = iterator.next();
  }

  return tokens;
}

/**
 * Resolves an unknown type of input and returns the corresponding TokenType.
 *
 * @param input - The input string to be resolved.
 * @return - The resolved TokenType object.
 */
function resolve_unknown_type(input: string): TokenType {
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

/**
 * Resolves a tag token based on the iterator and tag type.
 * @param iterator - The iterator to retrieve the next token.
 * @param tagType - The type of the tag.
 * @return - The resolved token type.
 */
function resolve_tag_token(iterator: Iterator<string, string>, tagType: TagType): TokenType {
  const tagValue = get_next_token(iterator);

  return {
    type: tagType,
    value: tagValue
  };
}

/**
 * Retrieves the next non-empty token from the given iterator.
 *
 * @param iterator - The iterator to retrieve tokens from.
 * @return - The next non-empty token.
 */
function get_next_token(iterator: Iterator<string, string>) {
  let current = iterator.next();

  while (!current.done) {
    if (current.value === "") {
      current = iterator.next();
    } else {
      return current.value;
    }
  }

  return "";
}
