<script lang="ts">
  import "../app.css";
  import "@skeletonlabs/skeleton/styles/all.css";
  import "../app.postcss";
  import { ActivityIcon, SettingsIcon, HomeIcon, KeyIcon } from "$lib/icons";
  import {
    computePosition,
    autoUpdate,
    flip,
    shift,
    offset,
    arrow,
  } from "@floating-ui/dom";
  import type { Writable } from "svelte/store";
  import { AppRail, AppRailTile } from "@skeletonlabs/skeleton";
  import { AppShell } from "@skeletonlabs/skeleton";
  import { writable } from "svelte/store";
  import { storePopup } from "@skeletonlabs/skeleton";

  const pageId: Writable<number> = writable(0);

  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<AppShell class="h-full">
  <svelte:fragment slot="sidebarLeft">
    <div class="h-full overflow-hidden">
      <AppRail selected={pageId}>
        <AppRailTile tag="a" href="/" label="Main" value={0}>
          <HomeIcon />
        </AppRailTile>
        <AppRailTile tag="a" href="/key-list" label="Keys" value={1}>
          <KeyIcon />
        </AppRailTile>
        <AppRailTile tag="a" href="/history" label="History" value={2}>
          <ActivityIcon />
        </AppRailTile>

        <AppRailTile
          slot="trail"
          tag="a"
          href="/settings"
          value={3}
          label="Settings"
        >
          <SettingsIcon />
        </AppRailTile>
      </AppRail>
    </div>
  </svelte:fragment>

  <slot />
</AppShell>

<style>
</style>
