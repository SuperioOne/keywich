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

  let activePage: number = 0;
  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<AppShell class="h-full">
  <svelte:fragment slot="header">
    <div class="grid grid-flow-col grid-cols-8 bg-surface-200-700-token">
      <div />
      <div class="col-span-6 flex flex-row justify-center">
        {#each MenuItems as item, index (index)}
          <a
            href={item.target}
            class:bg-initial={true}
            class="btn bg-initial bg-primary-hover-token rounded-none w-36 xs:w-fit"
            class:bg-primary-active-token={activePage === index}
            data-sveltekit-preload-data="hover"
            on:click={() => (activePage = index)}
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
      <div />
    </div>
  </svelte:fragment>
  <CenteredLayout>
    <slot />
  </CenteredLayout>
</AppShell>
