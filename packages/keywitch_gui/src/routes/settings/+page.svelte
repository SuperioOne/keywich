<script lang="ts">
  import {
    Accordion,
    AccordionItem,
    SlideToggle,
  } from "@skeletonlabs/skeleton";
  import { UserIcon, ToolIcon, TypeIcon } from "$lib/icons";
  import { lightSwitchController } from "$lib/settings/lightswitch";

  let value: number = 0;
</script>

<Accordion autocollapse>
  <AccordionItem open>
    <svelte:fragment slot="lead">
      <ToolIcon />
    </svelte:fragment>
    <svelte:fragment slot="summary">General</svelte:fragment>
    <svelte:fragment slot="content">
      <div class="p-2 flex flex-col gap-2">
        <div>
          <SlideToggle
            size="sm"
            name="theme-toggle"
            checked={$lightSwitchController}
            on:click={() => lightSwitchController.flip()}
          >
            {#if $lightSwitchController}
              Eye Bleach
            {:else}
              Dark
            {/if}
            Theme
          </SlideToggle>
        </div>

        <div>
          <span>Master Password</span>
          <select class="select">
            <option value="1">Never</option>
            <option value="2">10 Minutes</option>
            <option value="3">30 Minutes</option>
            <option value="4">1 Hour</option>
            <option value="5">Until session end</option>
          </select>
        </div>
      </div>
    </svelte:fragment>
  </AccordionItem>
  <hr />
  <AccordionItem>
    <svelte:fragment slot="lead">
      <UserIcon />
    </svelte:fragment>
    <svelte:fragment slot="summary">Data</svelte:fragment>
    <svelte:fragment slot="content">
      <div class="p-2 flex flex-col gap-2">
        <div>
          <SlideToggle size="sm" name="theme-toggle">Dark theme</SlideToggle>
        </div>
      </div>
    </svelte:fragment>
  </AccordionItem>
  <hr />
  <AccordionItem>
    <svelte:fragment slot="lead">
      <TypeIcon />
    </svelte:fragment>
    <svelte:fragment slot="summary">Import/Exports</svelte:fragment>
    <svelte:fragment slot="content">
      <div class="p-2 flex flex-col gap-2">
        <div>
          <SlideToggle size="sm" name="theme-toggle">Dark theme</SlideToggle>
        </div>
      </div>
    </svelte:fragment>
  </AccordionItem>
  <hr />
  <AccordionItem>
    <svelte:fragment slot="lead">
      <TypeIcon />
    </svelte:fragment>
    <svelte:fragment slot="summary">Shortcuts</svelte:fragment>
    <svelte:fragment slot="content">
      <div class="p-2 flex flex-col gap-2">
        <div>
          <SlideToggle size="sm" name="theme-toggle">Dark theme</SlideToggle>
        </div>
      </div>
    </svelte:fragment>
  </AccordionItem>
</Accordion>
