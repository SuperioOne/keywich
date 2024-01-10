<script lang="ts">
  import type {ThemeOptionType,} from "$lib";
  import {App, i18nStore, ThemeOptions, themeStore} from "$lib";
  import {SlideToggle} from "@skeletonlabs/skeleton";

  async function theme_color_change(event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    await App.Actions.set_theme_color(selectElement.value as ThemeOptionType ?? "crimson");
  }

  async function flip_mode(currentMode: boolean) {
    await App.Actions.set_theme_mode(!currentMode);
  }
</script>

<div class="flex flex-col gap-8">

  <div class="flex flex-row flex-wrap justify-between gap-2 w-full sm:w-auto">
    <div>
      <h2 class="font-bold">
        {i18nStore.getKey("i18:/settings/appearance/color-theme/title", "Color Theme")}
      </h2>
      <p class="font-light">
        <small>
          {i18nStore.getKey("i18:/settings/appearance/color-theme/desc", "Choose a color theme")}
        </small>
      </p>
    </div>
    <select class="select w-full sm:w-[200px]" on:change={theme_color_change}>
      {#each ThemeOptions as option (option)}
        <option selected={option === $themeStore.name} value={option}>{option}</option>
      {/each}
    </select>
  </div>

  <div class="flex flex-row flex-wrap justify-between items-center">
    <div>
      <h2 class="font-bold">
        {i18nStore.getKey("i18:/settings/appearance/theme/title", "Theme")}
      </h2>
      <p class="font-light">
        <small>
          <span>
            {#if $themeStore.isLight}
              {i18nStore.getKey("i18:/settings/appearance/theme/light", "Light Theme")}
            {:else}
              {i18nStore.getKey("i18:/settings/appearance/theme/dark", "Dark Theme")}
            {/if}
            </span>
        </small>
      </p>
    </div>
    <SlideToggle
        size="sm"
        name="theme-toggle"
        checked={$themeStore.isLight}
        on:click={() => flip_mode($themeStore.isLight)}
    >
    </SlideToggle>
  </div>

</div>