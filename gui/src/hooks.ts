import type {Reroute} from '@sveltejs/kit';
import {browser} from '$app/environment';

export const reroute: Reroute = ({url}) => {
  if (browser) {
    if (sessionStorage.getItem("unlocked") !== "1") {
      return "/unlock";
    } else {
      return url.pathname
    }
  } else {
    return url.pathname
  }
};