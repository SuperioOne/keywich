<script lang="ts">
  import { type ThemeOptionType } from "$lib";
  import { i18nStore, ThemeOptions, configStore } from "$lib";
  import { SlideToggle } from "@skeletonlabs/skeleton";

  async function theme_color_change(event: Event) {
    const select_element = event.target as HTMLSelectElement;
    configStore.set_theme(
      (select_element.value as ThemeOptionType) ?? "crimson",
    );
  }

  async function locale_change(event: Event) {
    const select_element = event.target as HTMLSelectElement;
    const locale = select_element.value as string;
    configStore.set_locale(locale);
    i18nStore.set_locale(locale);
  }
</script>

<div class="flex flex-col gap-8">
  <div
    class="flex flex-row flex-wrap sm:flex-nowrap justify-between gap-2 w-full sm:w-auto"
  >
    <div>
      <h2 class="font-bold">
        {$i18nStore.get_key(
          "i18:/settings/appearance/color-theme/title",
          "Color Theme",
        )}
      </h2>
      <p class="font-light">
        <small>
          {$i18nStore.get_key(
            "i18:/settings/appearance/color-theme/desc",
            "Choose a color theme",
          )}
        </small>
      </p>
    </div>
    <select class="select w-full sm:w-[200px]" on:change={theme_color_change}>
      {#each ThemeOptions as option (option)}
        <option selected={option === $configStore.color_theme} value={option}>
          {option}
        </option>
      {/each}
    </select>
  </div>

  <div class="flex flex-row flex-wrap justify-between items-center">
    <div>
      <h2 class="font-bold">
        {$i18nStore.get_key("i18:/settings/appearance/theme/title", "Theme")}
      </h2>
      <p class="font-light">
        <small>
          <span>
            {#if $configStore.is_light_theme}
              {$i18nStore.get_key(
                "i18:/settings/appearance/theme/light",
                "Light Theme",
              )}
            {:else}
              {$i18nStore.get_key(
                "i18:/settings/appearance/theme/dark",
                "Dark Theme",
              )}
            {/if}
          </span>
        </small>
      </p>
    </div>
    <SlideToggle
      size="sm"
      name="theme-toggle"
      checked={$configStore.is_light_theme}
      on:click={configStore.flip_mode}
    ></SlideToggle>
  </div>

  <div class="flex flex-row flex-wrap justify-between gap-2 w-full sm:w-auto">
    <div>
      <h2 class="font-bold">
        {$i18nStore.get_key(
          "i18:/settings/appearance/locale/title",
          "Language",
        )}
      </h2>
      <p class="font-light">
        <small>
          {$i18nStore.get_key(
            "i18:/settings/appearance/locale/desc",
            "Choose a language",
          )}
        </small>
      </p>
    </div>
    <select class="select w-full sm:w-[200px]" on:change={locale_change}>
      {#each i18nStore.available_locales as option (option)}
        <option selected={option === $i18nStore.current_locale} value={option}>
          {option}
        </option>
      {/each}
    </select>
  </div>
</div>

