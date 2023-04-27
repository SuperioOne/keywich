<script lang="ts">
  import "../app.css";
  import "@skeletonlabs/skeleton/styles/all.css";
  import "../app.postcss";
  import MenuItems from "./menu.config";
  import {
    computePosition,
    autoUpdate,
    flip,
    shift,
    offset,
    arrow,
  } from "@floating-ui/dom";
  import { AppShell, storePopup, AppBar } from "@skeletonlabs/skeleton";
  import { CenteredLayout } from "$lib/components";

  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<AppShell class="h-full">
  <svelte:fragment slot="header">
    <AppBar
      gridColumns="grid-cols-3"
      slotDefault="place-self-center"
      slotTrail="place-content-end"
      padding="p-0"
    >
      <svelte:fragment slot="lead"><div /></svelte:fragment>
      <div class="grid grid-flow-col auto-cols-max hover:auto-cols-min">
        {#each MenuItems as item, i (i)}
          <a
            class:bg-initial={true}
            href={item.target}
            class="btn bg-initial bg-primary-hover-token rounded-none w-full"
            data-sveltekit-preload-data="hover"
          >
            <div class="flex flex-col gap-1 align-middle justify-center w-full">
              <div class="flex justify-center">
                <svelte:component this={item.icon} />
              </div>
              <div class="xs:hidden flex justify-center">
                <span class="font-bold">
                  {item.label}
                </span>
              </div>
            </div>
          </a>
        {/each}
      </div>
      <svelte:fragment slot="trail"><div /></svelte:fragment>
    </AppBar>
  </svelte:fragment>
  <CenteredLayout>
    <slot />
  </CenteredLayout>
</AppShell>
