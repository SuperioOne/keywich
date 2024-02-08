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

  const modal_store = getModalStore();

  type OutType = PasswordOutputType | "UriEncoded";
  type DisplayData =
    { display: string, raw: string, type: OutType, state: "completed" }
    | { state: "loading" | "failed" }

  export let keyId: number;
  let data: DisplayData;

  async function get_password(output_type: OutType) {
    try {
      data = {state: "loading"};

      const result = await RPC.generate_password_from({
        content: "test",
        output_type: output_type === "UriEncoded" ? "Text" : output_type,
        profile_id: keyId
      });

      let display_data: string;
      switch (output_type) {
        case "Qr": {
          display_data = `data:image/svg+xml;base64,${btoa(result)}`;
          break;
        }
        case "UriEncoded": {
          display_data = encodeURIComponent(result);
          break;
        }
        case "Text":
        case "Base64":
        default:
          display_data = result;
          break;
      }

      data = {
        type: output_type,
        raw: result,
        state: "completed",
        display: display_data
      };

    } catch (err) {
      Log.error(err);
      data = {state: "failed"};
    }
  }

  async function save_qr(data: string) {
    const buffer = new TextEncoder().encode(data);
    await RPC.save_file(buffer);
  }
</script>

{#if $modal_store[0]}
  <div
      class="card p-6 flex flex-col items-center gap-5 w-full sm:w-modal-slim">
    {#if data?.state === "loading"}
      <ProgressRadial stroke={160} meter="stroke-primary-500" track="stroke-primary-500/30"/>
    {:else if data?.state === "completed"}
      {#if data.type === "Qr" }
        <div class="card w-full overflow-hidden aspect-square">
          <img width="100%" src={data.display} alt="qr"/>
        </div>
        <button
            class="btn variant-filled-primary cursor-pointer"
            on:click={() => save_qr(data.raw)}
        >
          <span><DownloadIcon/></span>
          <span>{
            i18nStore.get_key("i18:/generic/save", "Save")
          }</span>
        </button>
      {:else }
        <div class="w-full">
          <CodeBlock language={data.type} code={data.display}/>
        </div>
      {/if}
    {:else if data?.state === "failed"}
      <span class="text-error-300-600-token">Failed</span>
    {:else}
      <p class="font-bold w-full text-center">
        {$modal_store[0].title}
      </p>
      <hr class="!border-t-2 w-full"/>
      <div class="grid grid-cols-2 gap-2 w-full">
        <button
            on:click={() => get_password("Text")}
            type="button"
            class="aspect-square btn p-2 variant-filled-secondary flex flex-col gap-2 justify-center align-middle"
        >
          <TypeIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">TEXT</span>
        </button>
        <button
            on:click={() => get_password("Qr")}
            type="button"
            class="aspect-square btn p-2 variant-filled-secondary flex flex-col gap-2 justify-center align-middle"
        >
          <QrIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">QR</span>
        </button>
        <button
            on:click={() => get_password("Base64")}
            type="button"
            class="aspect-square btn p-2 variant-filled-secondary flex flex-col gap-2 justify-center align-middle"
        >
          <TerminalIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">BASE64</span>
        </button>
        <button
            on:click={() => get_password("UriEncoded")}
            type="button"
            class="aspect-square btn p-2 variant-filled-secondary flex flex-col gap-2 justify-center align-middle"
        >
          <LinkIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">URI Encoded</span>
        </button>
      </div>
    {/if}
  </div>
{/if}

