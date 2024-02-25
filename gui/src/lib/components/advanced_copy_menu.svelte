<script lang="ts">
  import DownloadIcon from "../icons/download.svelte"
  import LinkIcon from "../icons/link.svelte"
  import QrIcon from "../icons/qr.svelte"
  import TerminalIcon from "../icons/terminal.svelte"
  import TypeIcon from "../icons/type.svelte"
  import type {PasswordOutputType} from "@keywich/api";
  import {CodeBlock, getModalStore, ProgressRadial} from "@skeletonlabs/skeleton";
  import {Log} from "../logger";
  import {RPC} from "../rpc";
  import {i18nStore} from "../stores/i18n_store";
  import {getToastStore} from "../stores/extended_toast_store";
  import {is_error_response} from "@keywich/api/utils";
  import {onDestroy} from "svelte";

  type OutType = PasswordOutputType | "UriEncoded";
  type DisplayData =
    { state: "completed", display_data: string, type: "Json" | "Base64" | "Text" | "UriEncoded" } |
    { state: "completed", display_data: string, raw_data: Uint8Array, type: "Qr" } |
    { state: "loading" } |
    { state: "failed", message: string }

  export let keyId: number;

  const modal_store = getModalStore();
  const toast_store = getToastStore()
  let display: DisplayData;

  onDestroy(() => try_revoke_qr());

  function try_revoke_qr() {
    if (display && display.state === "completed" && display.type === "Qr" && display.display_data) {
      Log.debug("Removed QR URL");
      URL.revokeObjectURL(display.display_data);
    }
  }

  async function get_password(output_type: OutType) {
    try {
      display = {state: "loading"};

      const result = await RPC.generate_password_from({
        output_type: output_type === "UriEncoded" ? "Text" : output_type,
        profile_id: keyId
      });

      try_revoke_qr();

      switch (output_type) {
        case "Qr": {
          const encoded_svg = new TextEncoder().encode(result);
          display = {
            type: output_type,
            raw_data: encoded_svg,
            state: "completed",
            display_data: URL.createObjectURL(new Blob([encoded_svg], {type: 'image/svg+xml'}))
          };
          break;
        }
        case "UriEncoded": {
          display = {
            type: output_type,
            state: "completed",
            display_data: encodeURIComponent(result)
          };
          break;
        }
        case "Text":
        case "Base64":
        default:
          display = {
            type: output_type,
            state: "completed",
            display_data: result
          };
          break;
      }
    } catch (err) {
      Log.error(err);
      display = {state: "failed", message: "Key generation failed."};
    }
  }

  async function save_qr(data: Uint8Array) {
    try {
      await RPC.save_file(data);
      toast_store.trigger_success($i18nStore.get_key("i18:/actions/advanced-copy/qr-saved", "File saved."));
    } catch (err) {

      Log.error(err);
      if (is_error_response(err)) {
        toast_store.trigger_error($i18nStore.get_key(`i18:/errors/${err.code}`, "Save Failed."));
      } else {
        toast_store.trigger_error($i18nStore.get_key("i18:/actions/advanced-copy/unknown-error", "Unexpected error."));
      }
    }
  }
</script>

{#if $modal_store[0]}
  <div
      class="card p-6 flex flex-col items-center gap-5 w-full sm:w-modal-slim">
    {#if display?.state === "loading"}
      <ProgressRadial stroke={160} meter="stroke-primary-500" track="stroke-primary-500/30"/>

    {:else if display?.state === "completed"}
      {#if display.type === "Qr" }
        {@const display_url = display.display_data}
        {@const qr_data = display.raw_data}
        <div class="card w-full overflow-hidden aspect-square">
          <img width="100%" src={display_url} alt="qr"/>
        </div>
        <button
            class="btn variant-filled-primary cursor-pointer"
            on:click={() => save_qr(qr_data)}
        >
          <span><DownloadIcon/></span>
          <span>{$i18nStore.get_key("i18:/generic/save", "Save")}</span>
        </button>

      {:else }
        <div class="w-full">
          <CodeBlock language={display.type} code={display.display_data}/>
        </div>
      {/if}

    {:else if display?.state === "failed"}
      <span class="text-error-300-600-token">{display.message}</span>

    {:else}
      <p class="font-bold w-full text-center">
        {$modal_store[0].title}
      </p>
      <hr class="!border-t-2 w-full"/>
      <div class="grid grid-cols-2 gap-2 w-full">
        <button
            on:click={() => get_password("Text")}
            type="button"
            class="aspect-square btn p-2 variant-glass-secondary flex flex-col gap-2 justify-center align-middle"
        >
          <TypeIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">TEXT</span>
        </button>
        <button
            on:click={() => get_password("Qr")}
            type="button"
            class="aspect-square btn p-2 variant-glass-secondary flex flex-col gap-2 justify-center align-middle"
        >
          <QrIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">QR</span>
        </button>
        <button
            on:click={() => get_password("Base64")}
            type="button"
            class="aspect-square btn p-2 variant-glass-secondary flex flex-col gap-2 justify-center align-middle"
        >
          <TerminalIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">BASE64</span>
        </button>
        <button
            on:click={() => get_password("UriEncoded")}
            type="button"
            class="aspect-square btn p-2 variant-glass-secondary flex flex-col gap-2 justify-center align-middle"
        >
          <LinkIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">URI Encoded</span>
        </button>
      </div>
    {/if}
  </div>
{/if}

