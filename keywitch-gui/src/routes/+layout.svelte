<script lang="ts">
  import '@skeletonlabs/skeleton/themes/theme-crimson.css';
  import "@skeletonlabs/skeleton/styles/all.css";
  import "../app.postcss";
  import "../app.css"

  import type {Writable} from "svelte/store";
  import {AppRail, AppRailTile} from "@skeletonlabs/skeleton";
  import {AppShell} from '@skeletonlabs/skeleton';
  import {writable} from "svelte/store";

  const pageId: Writable<number> = writable(0);
</script>

<style>
    html, body {
        @apply h-full overflow-hidden;
    }
</style>

<AppShell class="h-screen">
    <svelte:fragment slot="sidebarLeft">
        <AppRail selected={pageId}>
            <AppRailTile tag="a" href="/" label="Home" value={0}>(icon)</AppRailTile>
            <AppRailTile tag="a" href="/history" label="History" value={1}>(icon)</AppRailTile>
            <AppRailTile tag="a" href="/settings" label="Settings" value={2}>(icon)</AppRailTile>
        </AppRail>
    </svelte:fragment>
    <div class="p-10 flex align-middle justify-center">
        <div class="flex-row w-full max-w-screen-lg">
            <slot/>
        </div>
    </div>
</AppShell>

