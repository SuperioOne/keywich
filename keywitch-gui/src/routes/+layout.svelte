<script lang="ts">
  import "../app.css";
  import "@skeletonlabs/skeleton/styles/all.css";
  import "../app.postcss";
  import { ActivityIcon, SettingsIcon, HomeIcon, KeyIcon } from "$lib/icons";

  import type { Writable } from "svelte/store";
  import { AppRail, AppRailTile } from "@skeletonlabs/skeleton";
  import { AppShell } from "@skeletonlabs/skeleton";
  import { writable } from "svelte/store";

  const pageId: Writable<number> = writable(0);
</script>

<AppShell class="h-screen">
  <svelte:fragment slot="sidebarLeft">
    <AppRail selected={pageId}>
      <div slot="lead" class="flex justify-center align-middle py-7">
        <HomeIcon />
      </div>
      <AppRailTile tag="a" href="/" label="Keys" value={0}>
        <KeyIcon />
      </AppRailTile>
      <AppRailTile tag="a" href="/history" label="History" value={1}>
        <ActivityIcon />
      </AppRailTile>
      <AppRailTile slot="trail" tag="a" href="/settings" value={2}>
        <SettingsIcon />
      </AppRailTile>
    </AppRail>
  </svelte:fragment>
  <div class="p-10 flex align-middle justify-center">
    <div class="flex-row w-full max-w-screen-lg">
      <slot />
    </div>
  </div>
</AppShell>

<style>
  html,
  body {
    @apply h-full overflow-hidden;
  }
</style>
