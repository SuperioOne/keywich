import type {SearchQuery} from "@keywich/api";
import type {PageLoad} from './$types';
import type {TokenType, TokenTypeName} from "$lib";
import {RPC} from "$lib";

export const load: PageLoad = async ({url}) => {
  const filter = get_filter(url);

  const keys = filter?.keyFilter
    ? await RPC.search_keys(filter.keyFilter)
    : await RPC.get_keys();

  return {
    keys: keys,
    tokens: filter?.tokens
  };
};

type FilterObject = {
  keyFilter: SearchQuery;
  tokens: TokenType[];
}

function get_filter(url: URL): FilterObject | undefined {
  const tag = url.searchParams.getAll("t");
  const username = url.searchParams.getAll("u");
  const domain = url.searchParams.getAll("d");
  const terms = url.searchParams.getAll("s");

  if (tag.length > 0 || username.length > 0 || domain.length > 0 || terms.length > 0) {

    const segments: FilterSegment[] = [
      {
        type: "tag",
        tokens: tag
      },
      {
        type: "term",
        tokens: terms
      },
      {
        type: "domain",
        tokens: domain
      },
      {
        type: "username",
        tokens: username
      },
    ];

    return {
      keyFilter: {
        username,
        tag,
        domain,
      },
      tokens: to_tokens(segments)
    }
  } else {
    return undefined;
  }
}

type FilterSegment = {
  type: TokenTypeName;
  tokens: string[];
}

function to_tokens(segments: FilterSegment[]) {
  const tokens: TokenType[] = [];

  for (const filterSegment of segments) {
    for (const token of filterSegment.tokens) {
      tokens.push({type: filterSegment.type, value: token});
    }
  }

  return tokens;
}
