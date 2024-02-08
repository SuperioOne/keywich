<script lang="ts">
  import type {PageData} from "./$types";
  import {i18nStore} from "$lib";
  import {ListBox, ListBoxItem} from "@skeletonlabs/skeleton";

  import AppearanceSettings from "./settings_appearance.svelte";
  import CharsetSettings from "./settings_charsets.svelte";

  export let data: PageData;
  let selection: string = data.section ?? "appearance";

</script>

<div class="flex flex-row flex-wrap sm:flex-nowrap gap-2 w-full">
  <div class="w-full sm:w-fit p-4 rounded-none border-b-[1px] border-surface-600 sm:border-r-[1px] sm:border-b-[0px]">
    <ListBox active="variant-filled-secondary" spacing="space-y-3">
      <ListBoxItem bind:group={selection} name="medium" value="appearance">
        {i18nStore.get_key("i18:/settings/nav/appearance", "Appearance")}
      </ListBoxItem>
      <ListBoxItem bind:group={selection} name="medium" value="charsets">
        {i18nStore.get_key("i18:/settings/nav/charsets", "Charsets")}
      </ListBoxItem>
    </ListBox>
  </div>

  <div class="flex flex-col gap-8 w-full">
    {#if selection === "appearance"}
      <section class="p-4">
        <AppearanceSettings/>
      </section>

    {:else if selection === "charsets" }
      <section class="p-4">
        <CharsetSettings charsets={data.charsets}/>
      </section>
    {/if}
  </div>
</div>
