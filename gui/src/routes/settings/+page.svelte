<script lang="ts">
  import {
    Accordion,
    AccordionItem,
    SlideToggle,
  } from "@skeletonlabs/skeleton";
  import {get_app_context, i18nStore, lightSwitchController} from "$lib";

  const context = get_app_context();

  let value: number = 0;

  function on_theme_swap(event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    document.body.setAttribute("data-theme", selectElement.value ?? "crimson");
  }
</script>

<Accordion autocollapse>
  <AccordionItem open>
    <svelte:fragment slot="lead">
    </svelte:fragment>
    <svelte:fragment slot="summary">General</svelte:fragment>
    <svelte:fragment slot="content">
      <div class="p-2 flex flex-col gap-2">
        <div>
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

        <div>
          <span>Theme</span>
          <select class="select" on:change={on_theme_swap}>
            <option value="crimson">crimson</option>
            <option value="skeleton">skeleton</option>
            <option value="hamlindigo">hamlindigo</option>
            <option value="wintry">wintry</option>
            <option value="rocket">rocket</option>
            <option value="vintage">vintage</option>
          </select>
        </div>
      </div>
    </svelte:fragment>
  </AccordionItem>
  <hr/>
  <AccordionItem>
    <svelte:fragment slot="lead">
    </svelte:fragment>
    <svelte:fragment slot="summary">Data</svelte:fragment>
    <svelte:fragment slot="content">
      <div class="p-2 flex flex-col gap-2">
        <div>
          <SlideToggle size="sm" name="theme-toggle" on:click={lightSwitchController.flip}>Dark theme</SlideToggle>
        </div>
      </div>
    </svelte:fragment>
  </AccordionItem>
  <hr/>
  <AccordionItem>
    <svelte:fragment slot="lead">
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
  <hr/>
  <AccordionItem>
    <svelte:fragment slot="lead">
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
