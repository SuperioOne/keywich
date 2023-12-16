<script lang="ts">
  import {
    Accordion,
    AccordionItem,
    SlideToggle,
  } from "@skeletonlabs/skeleton";
  import {i18nStore, theme_store, ThemeOptions,} from "$lib";
  import type {ThemeOptionType} from "$lib";

  function on_theme_swap(event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    theme_store.set_theme(selectElement.value as ThemeOptionType ?? "crimson");
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
          <span>Theme</span>
          <select class="select" on:change={on_theme_swap}>
            {#each ThemeOptions as option (option)}
              <option selected={option === $theme_store.name} value={option}>{option}</option>
            {/each}
          </select>
        </div>

        <div>
          <SlideToggle
            size="sm"
            name="theme-toggle"
            checked={$theme_store.isLight}
            on:click={theme_store.flip_mode}
          >
            {#if $theme_store.isLight}
              Light Mode
            {:else}
              Dark Mode
            {/if}
          </SlideToggle>
        </div>
      </div>
    </svelte:fragment>
  </AccordionItem>
  <hr/>
  <AccordionItem>
    <svelte:fragment slot="lead">
    </svelte:fragment>
    <svelte:fragment slot="summary">Charsets</svelte:fragment>
    <svelte:fragment slot="content">
      <div class="p-2 flex flex-col gap-2">

      </div>
    </svelte:fragment>
  </AccordionItem>
</Accordion>
