<script lang="ts">
  import DownloadIcon from "$lib/icons/download.svelte"
  import QrIcon from "$lib/icons/qr.svelte"
  import LinkIcon from "$lib/icons/link.svelte"
  import RPC from "@keywitch/memory_rpc";
  import TerminalIcon from "$lib/icons/terminal.svelte"
  import TypeIcon from "$lib/icons/type.svelte"
  import type {PasswordOutputType} from "@keywitch/rpc";
  import {CodeBlock, getModalStore, ProgressRadial} from "@skeletonlabs/skeleton";
  import {Log} from "$lib/logger";
  import {i18nStore} from "$lib/stores/i18n_store";

  const modalStore = getModalStore();

  type DisplayData =
    { display: string, raw: string, type: PasswordOutputType, state: "completed" }
    | { state: "loading" | "failed" }

  export let keyId: number;
  let data: DisplayData;

  async function get_password(output_type: PasswordOutputType) {
    try {
      data = {state: "loading"};
      const result = await RPC.KeyMetadata.generate_password(keyId, output_type);

      if (result.success === false) {
        Log.error(result.error);
        data = {state: "failed"};
        return;
      }

      let displayData: string;
      switch (output_type) {
        case "qr": {
          displayData = `data:image/svg+xml;base64,${btoa(result.data)}`;
          break;
        }
        case "text":
        case "base64":
        case "uri":
        default:
          displayData = result.data;
          break;
      }

      data = {
        type: output_type,
        raw: result.data,
        state: "completed",
        display: displayData
      };

    } catch (err) {
      Log.error(err);
      data = {state: "failed"};
    }
  }

  async function save_qr(data: string) {
    const buffer = new TextEncoder().encode(data);
    await RPC.Utility.save_file(buffer);
  }
</script>

{#if $modalStore[0]}
  <div
    class="card p-6 flex flex-col items-center gap-5 w-full sm:w-modal-slim">
    {#if data?.state === "loading"}
      <ProgressRadial stroke={160} meter="stroke-primary-500" track="stroke-primary-500/30"/>
    {:else if data?.state === "completed"}
      {#if data.type === "qr" }
        <div class="card w-full overflow-hidden aspect-square">
          <img width="100%" src={data.display} alt="qr image"/>
        </div>
        <button
          class="btn variant-soft-primary cursor-pointer"
          on:click={() => save_qr(data.raw)}
        >
          <span><DownloadIcon/></span>
          <span>{
            i18nStore.getKey("i18:/generic/save", "Save")
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
        {$modalStore[0].title}
      </p>
      <hr class="!border-t-2 w-full"/>
      <div class="grid grid-cols-2 gap-2 w-full">
        <button
          on:click={() => get_password("text")}
          type="button"
          class="aspect-square btn p-2 variant-soft-primary flex flex-col gap-2 justify-center align-middle"
        >
          <TypeIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">TEXT</span>
        </button>
        <button
          on:click={() => get_password("qr")}
          type="button"
          class="aspect-square btn p-2 variant-soft-primary flex flex-col gap-2 justify-center align-middle"
        >
          <QrIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">QR</span>
        </button>
        <button
          on:click={() => get_password("base64")}
          type="button"
          class="aspect-square btn p-2 variant-soft-primary flex flex-col gap-2 justify-center align-middle"
        >
          <TerminalIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">BASE64</span>
        </button>
        <button
          on:click={() => get_password("uri")}
          type="button"
          class="aspect-square btn p-2 variant-soft-primary flex flex-col gap-2 justify-center align-middle"
        >
          <LinkIcon size={48}/>
          <span class="font-mono text-sm text-center w-full !m-0">URI Encoded</span>
        </button>
      </div>
    {/if}
  </div>
{/if}

