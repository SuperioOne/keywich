<script lang="ts">
  import {RPC, type ThemeOptionType, to_promise} from "$lib";
  import {Accordion, AccordionItem, SlideToggle,} from "@skeletonlabs/skeleton";
  import {themeStore, ThemeOptions} from "$lib";

  function on_theme_swap(event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    themeStore.set_theme(selectElement.value as ThemeOptionType ?? "crimson");
  }
</script>

<div class="card p-4">
  <Accordion>
    <AccordionItem open>
      <svelte:fragment slot="summary"><h1 class="font-bold text-xl">Appearance</h1></svelte:fragment>
      <svelte:fragment slot="content">
        <div class="p-4 flex flex-col gap-8">

          <div class="flex flex-row flex-wrap justify-between items-center">
            <span>Theme</span>
            <select class="select w-fit" on:change={on_theme_swap}>
              {#each ThemeOptions as option (option)}
                <option selected={option === $themeStore.name} value={option}>{option}</option>
              {/each}
            </select>
          </div>

          <div class="flex flex-row flex-wrap justify-between items-center">
          <span>
            {#if $themeStore.isLight}
              Flashbang!
            {:else}
              Dark Mode
            {/if}
          </span>
            <SlideToggle
                size="sm"
                name="theme-toggle"
                checked={$themeStore.isLight}
                on:click={themeStore.flip_mode}
            >
            </SlideToggle>
          </div>

        </div>
      </svelte:fragment>
    </AccordionItem>
    <hr/>
    <AccordionItem open>
      <svelte:fragment slot="summary"><h1 class="font-bold text-xl">Charsets</h1></svelte:fragment>
      <svelte:fragment slot="content">
        <div class="p-4 flex flex-col gap-8">

          <div class="flex flex-row flex-wrap justify-between items-center">
            <span>Charsets</span>

            <ul>
              {#await to_promise(RPC.Charset.get_charsets())}
                <li>
                  Loading...
                </li>
              {:then charset_list}
                {#each charset_list as charset (charset.id)}
                  <li>
                    {charset.name}
                  </li>
                {/each}
              {:catch err}
                <li>
                  Failed
                </li>
              {/await}
            </ul>
          </div>

        </div>
      </svelte:fragment>
    </AccordionItem>
  </Accordion>
</div>