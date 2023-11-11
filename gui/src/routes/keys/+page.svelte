<script lang="ts">
  import AdvancedCopy from "./advanced_copy.svelte";
  import ClipboardIcon from "$lib/icons/clipboard.svelte";
  import EditIcon from "$lib/icons/edit.svelte";
  import FilterIcon from "$lib/icons/filter.svelte";
  import KeyForm from "./create_key_form.svelte";
  import KeyIcon from "$lib/icons/key.svelte";
  import PlusCircleIcon from "$lib/icons/plus-circle.svelte";
  import RPC from "@keywitch/memory_rpc";
  import StarIcon from "$lib/icons/star.svelte";
  import TrashIcon from "$lib/icons/trash-2.svelte";
  import type {KeyMetadataItem} from "@keywitch/rpc";
  import type {ModalActionResult} from "./types";
  import type {PageData} from "./$types";
  import {ModalAction} from "./types";
  import {getExtendedToastStore, Log} from "$lib";
  import {getModalStore} from "@skeletonlabs/skeleton";
  import {invalidateAll} from "$app/navigation";

  export let data: PageData;
  const modalStore = getModalStore();
  const toastStore = getExtendedToastStore();

  async function new_key() {
    const response = await new Promise<ModalActionResult>((resolve) => {
      modalStore.trigger({
        component: {
          ref: KeyForm,
        },
        title: "Create New Key",
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult) => resolve(r),
      });
    });

    if (response.type === ModalAction.submitted) {
      await invalidateAll();
      toastStore.trigger_success("New key created successfully.");
    }
  }

  async function flip_pin(item: KeyMetadataItem) {
    const rpcAction = item.pinned
      ? RPC.KeyMetadata.unpin_key
      : RPC.KeyMetadata.pin_key;

    const result = await rpcAction(item.id);

    if (result.success) {
      await invalidateAll();
    } else {
      Log.error(result.error);
      toastStore.trigger_error("Unable to pin");
    }
  }

  async function quick_copy(keyDetails: KeyMetadataItem) {
    try {
      const result = await RPC.KeyMetadata.generate_password(keyDetails.id, "text");
      if (result.success) {
        await navigator.clipboard.writeText(result.data);
        toastStore.trigger_success("Key copied to clipboard.");
      } else {
        Log.error(result.error);
        toastStore.trigger_error("Key generation failed. See logs for more details.");
      }
    } catch (err) {
      Log.error(err as Error);
      toastStore.trigger_error("Key generation failed. See logs for more details.");
    }
  }

  async function advanced_copy(keyDetails: KeyMetadataItem) {
    await new Promise<ModalActionResult>((resolve) => {
      modalStore.trigger({
        component: {
          ref: AdvancedCopy,
          props: {
            keyId: keyDetails.id
          }
        },
        title: "Advanced options",
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult) => resolve(r),
      });
    });
  }

  async function delete_key(id: number) {
    const confirmation = await new Promise((resolve) => {
      modalStore.trigger({
        type: "confirm",
        title: "Confirm Action",
        body: `Are you sure to delete key?`,
        buttonTextConfirm: "Delete",
        response: (r: boolean) => resolve(r),
      });
    });

    if (confirmation) {
      const result = await RPC.KeyMetadata.remove_key(id);

      if (result.success) {
        await invalidateAll();
        toastStore.trigger_warning("Key removed from store.");
      } else {
        Log.warn(result.error);
        toastStore.trigger_error("Unable to remove key.");
      }
    }
  }
</script>

<div class="flex gap-6 flex-col">

  <div class="grid grid-cols-2 gap-6">
    <div class="col-span-full sm:col-span-1 flex flex-row flex-wrap gap-2">
      <button
        on:click={new_key}
        type="button"
        class="btn variant-soft-primary w-full sm:w-auto"
      >
        <PlusCircleIcon/>
        <span class="font-bold"> Create </span>
      </button>
    </div>
    <div class="col-span-full sm:col-span-1 flex flex-row flex-wrap gap-2 justify-end">
      <button
        on:click={new_key}
        type="button"
        class="btn variant-soft w-full sm:w-auto"
      >
        <FilterIcon/>
        <span class="font-bold"> Sort </span>
      </button>
      <button
        on:click={new_key}
        type="button"
        class="btn variant-soft w-full sm:w-auto"
      >
        <FilterIcon/>
        <span class="font-bold"> Filter </span>
      </button>
    </div>
  </div>

  <dl class="flex flex-col gap-1">
    {#each data.keys as row (row.id)}
      <div role="none"
           class="card grid grid-cols-5 sm:grid-cols-8 md:grid-cols-12 gap-3 w-full p-4 items-stretch">

        <div class="flex col-span-1 items-center">
          <button
            type="button"
            class="text-primary-500 bg-surface-200-700-token p-3 btn btn-icon-xl"
          >
            {#if row.custom_icon}
              <img src={row.custom_icon} alt="Missing Icon"/>
            {:else}
              <KeyIcon/>
            {/if}
          </button>
        </div>

        <div class="col-span-3 sm:col-span-4 md:col-span-8 flex flex-row sm:gap-8 gap-3 items-center">
          <div class="w-full overflow-hidden flex flex-col gap-3">
            <div>
              <dt class="font-bold">{row.user_name}</dt>
              <dd class="text-sm font-thin truncate">
                {row.domain}
              </dd>
            </div>
            <div class="flex flex-wrap gap-1 min-h-[24px]">
              {#each row.tags as tag (tag)}
                <span
                  class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
                >
                  {tag}
                </span>
              {/each}
            </div>
          </div>
        </div>
        <div
          class="col-span-1 sm:col-span-3 flex flex-col sm:flex-row gap-2 sm:gap-4 justify-end items-center"
        >
          <button
            type="button"
            on:contextmenu|preventDefault={() => advanced_copy(row)}
            on:auxclick|preventDefault={() => quick_copy(row)}
            on:click|preventDefault={() => quick_copy(row)}
            class="btn btn-sm variant-soft-primary btn-icon-base h-fit"
          >
            <ClipboardIcon/>
          </button>
          <button
            type="button"
            class="btn btn-sm btn-icon-base h-fit"
            class:variant-ghost-warning={row.pinned}
            class:variant-soft={!row.pinned}
            on:click|preventDefault={() => flip_pin(row)}
          >
            <StarIcon/>
          </button>
          <button
            type="button"
            class="btn btn-sm variant-soft-secondary btn-icon-base h-fit"
          >
            <EditIcon/>
          </button>
          <button
            on:click={async () => delete_key(row.id)}
            type="button"
            class="btn btn-sm variant-soft-error btn-icon-base h-fit"
          >
            <TrashIcon/>
          </button>
        </div>
      </div>
    {/each}
  </dl>
</div>
