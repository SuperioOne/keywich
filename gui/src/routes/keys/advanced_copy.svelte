<script lang="ts">
  import CodeIcon from "$lib/icons/code.svelte"
  import QrIcon from "$lib/icons/qr.svelte"
  import TerminalIcon from "$lib/icons/terminal.svelte"
  import TypeIcon from "$lib/icons/type.svelte"
  import DownloadIcon from "$lib/icons/download.svelte"
  import {CodeBlock, getModalStore, ProgressRadial} from "@skeletonlabs/skeleton";
  import type {PasswordOutputType} from "$lib";
  import {getExtendedToastStore, Log, RPC} from "$lib";

  const modalStore = getModalStore();
  const toastStore = getExtendedToastStore();
  export let keyId: number;
  let displayData: Promise<{ type: PasswordOutputType, data: string }> | undefined = undefined;

  async function get_password(output_type: PasswordOutputType) {
    displayData = new Promise((resolve, reject) => {
      RPC.generate_password(keyId, output_type)
        .then((data) => {
          resolve({type: output_type, data: data})
        })
        .catch((err) => {
          Log.error(err);
          reject(err);
        });
    });
  }

  async function save_qr(data: string) {
    const buffer = new TextEncoder().encode(data);
    await RPC.save_file(buffer);
    toastStore.trigger_success("QR image saved.");
  }
</script>

{#if $modalStore[0]}
  <div
    class="card variant-filled-surface bg-surface-100-800-token p-6 flex flex-col items-center gap-5 w-full sm:w-modal-slim">
    {#if displayData}
      {#await displayData}
        <ProgressRadial stroke={160} meter="stroke-primary-500" track="stroke-primary-500/30"/>
      {:then password}
        {#if password.type === "qr" }
          <div class="card overflow-hidden">
            {@html password.data}
          </div>
          <button
            class="btn variant-soft-primary cursor-pointer"
            on:click={() => save_qr(password.data)}
          >
            <span><DownloadIcon/></span>
            <span>Save</span>
          </button>
        {:else }
          <div class="w-full">
            <CodeBlock language={password.type} code={password.data}/>
          </div>
        {/if}
      {:catch err}
        <div class="text-error-300-600-token">Failed to load password: <span>{err?.message}</span></div>
      {/await}
    {:else}
      <p class="font-bold w-full text-center">
        {$modalStore[0].title}
      </p>
      <hr class="!border-t-2 w-full"/>
      <div class="grid grid-cols-2 gap-3 w-full">
        <button
          on:click={() => get_password("text")}
          type="button"
          class="btn p-2 variant-soft-primary flex flex-col gap-1 justify-center align-middle"
        >
          <TypeIcon/>
          <span class="font-mono text-xs text-center w-full !m-0">TEXT</span>
        </button>
        <button
          on:click={() => get_password("qr")}
          type="button"
          class="btn p-2 variant-soft-primary flex flex-col gap-1 justify-center align-middle"
        >
          <QrIcon/>
          <span class="font-mono text-xs text-center w-full !m-0">QR</span>
        </button>
        <button
          on:click={() => get_password("json")}
          type="button"
          class="btn p-2 variant-soft-primary flex flex-col gap-1 justify-center align-middle"
        >
          <CodeIcon/>
          <span class="font-mono text-xs text-center w-full !m-0">JSON</span>
        </button>
        <button
          on:click={() => get_password("base64")}
          type="button"
          class="btn p-2 variant-soft-primary flex flex-col gap-1 justify-center align-middle"
        >
          <TerminalIcon/>
          <span class="font-mono text-xs text-center w-full !m-0">BASE64</span>
        </button>
      </div>
    {/if}
  </div>
{/if}