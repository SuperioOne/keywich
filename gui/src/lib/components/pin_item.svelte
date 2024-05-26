<script lang="ts">
  import KeyIcon from "./key_icon.svelte";
  import type { KeyItem } from "../api";
  import { Api } from "../api";
  import { i18nStore } from "$lib/stores/i18n_store";
  import { Log } from "$lib/logger";
  import { getToastStore } from "$lib/stores/extended_toast_store";

  export let item: KeyItem;
  const toast_store = getToastStore();

  async function quick_copy() {
    try {
      const result = await Api.generate_password_from({
        profile_id: item.id,
        output_type: "Text",
      });

      await Api.copy_to_clipboard(result);
      toast_store.trigger_success(
        $i18nStore.get_key(`i18:/actions/copy-key/msg/success`, "Key copied."),
      );
    } catch (err) {
      Log.error(err);
      toast_store.trigger_error(
        $i18nStore.get_key(
          `i18:/actions/copy-key/msg/error`,
          "Key generation failed.",
        ),
      );
    }
  }
</script>

<div class="block card card-hover overflow-hidden">
  <a
    href="#"
    class="inline-block pin-btn variant-glass-secondary p-5 w-full h-full"
    on:click|preventDefault={quick_copy}
  >
    <div
      class="flex flex-col gap-2 justify-center items-center overflow-hidden"
    >
      <div
        class="text-primary-500 p-0 btn btn-icon-xl w-16 h-16 overflow-hidden"
      >
        <KeyIcon icon={item.custom_icon} size={64} />
      </div>
      <div class="font-bold truncate w-[90%] text-center">{item.username}</div>
      <div class="font-light text-xs truncate w-[90%] text-center">
        {item.domain}
      </div>
    </div>
  </a>
</div>
