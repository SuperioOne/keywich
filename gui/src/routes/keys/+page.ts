import RPC from "@keywitch/memory_rpc";
import type {KeyFilter} from "@keywitch/rpc";
import type {PageLoad} from './$types';
import type {TokenType, TokenTypeName} from "$lib";
import {Log} from "$lib";

export const load: PageLoad = async ({url, params, data}) => {
  const filter = get_filter(url);
  const result = await RPC.KeyMetadata.get_key_collection(filter?.keyFilter);

  if (result.success) {
    return {
      keys: result.data,
      tokens: filter?.tokens
    };
  } else {
    Log.error(result.error);
    return {
      keys: [],
      tokens: filter?.tokens
    }
  }
};

type FilterObject = {
  keyFilter: KeyFilter;
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
        searchTokens: terms
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
