<script lang="ts">
  import {
    Accordion,
    AccordionItem,
    SlideToggle,
  } from "@skeletonlabs/skeleton";
  import { UserIcon, ToolIcon, TypeIcon } from "$lib/icons";
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
          <SlideToggle size="sm" name="theme-toggle">Dark theme</SlideToggle>
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
