<script lang="ts">
  import {
    ClipboardIcon,
    TrashIcon,
    KeyIcon,
    PlusCircleIcon,
    StarIcon,
    EditIcon,
    TypeIcon,
    TerminalIcon,
    CodeIcon,
    QrIcon,
  } from "$lib/icons";
  import type {ModalActionResult} from "./types";
  import type {PageData} from "./$types";
  import {ModalAction} from "./types";
  import {RPC} from "$lib";
  import type {KeyMetadataItem} from "$lib";
  import {default as KeyForm} from "./create_key_form.svelte";
  import {default as AdvancedCopy} from "./advanced_copy.svelte";
  import {getModalStore, getToastStore, popup} from "@skeletonlabs/skeleton";
  import {invalidateAll} from "$app/navigation";

  export let data: PageData;
  const modalStore = getModalStore();
  const toastStore = getToastStore();

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

    if (response.type === ModalAction.submit) {
      await RPC.add_key(response.data);
      await invalidateAll();

      toastStore.trigger({
        message: "New key created successfully.",
        background: "variant-filled-success",
        timeout: 3000,
      });
    }
  }

  async function quick_copy(id: number, event: Event) {
    try {
      const key = await RPC.calculate_password(id, "text");
      await navigator.clipboard.writeText(key);
      toastStore.trigger({
        message: "Key copied",
        background: "variant-filled-success",
        timeout: 1500,
      });
    } catch (err) {
      toastStore.trigger({
        message: "Unable to get key",
        background: "variant-filled-error",
        timeout: 3000,
      });
    }
  }

  async function advanced_copy(rowInfo: KeyMetadataItem, event: Event) {
    await new Promise<ModalActionResult>((resolve) => {
      modalStore.trigger({
        component: {
          ref: AdvancedCopy,
          props: {
            keyId: rowInfo.id
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
      const success = await RPC.remove_key(id);
      if (success) {
        await invalidateAll();
        toastStore.trigger({
          message: "Key removed from store",
          background: "variant-filled-warning",
          timeout: 3000,
        });
      }
    }
  }
</script>

<div class="flex gap-6 flex-col">
  <div class="flex flex-row gap-1">
    <button
      on:click={new_key}
      type="button"
      class="btn variant-soft-primary w-full sm:w-auto"
    >
      <PlusCircleIcon/>
      <span class="font-bold"> Create </span>
    </button>
  </div>

  <dl class="flex flex-col gap-1">
    {#each data.keys as row (row.id)}
      <div role="none" class="card grid grid-cols-4 gap-3 w-full p-4">
        <div class="col-span-3 flex flex-row sm:gap-8 gap-3 items-center">
          <div
            class="text-primary-500 bg-surface-200-700-token p-3 btn-sm btn h-fit"
          >
            <KeyIcon/>
          </div>
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
          class="flex flex-col md:flex-row flex-wrap gap-1 sm:gap-2 justify-end items-center"
        >
          <button
            type="button"
            on:contextmenu|preventDefault={(ev) => advanced_copy(row, ev)}
            on:auxclick|preventDefault={(ev) => quick_copy(row, ev)}
            on:click|preventDefault={(ev) => quick_copy(row, ev)}
            class="btn btn-sm variant-soft-primary btn-icon-base h-fit"
          >
            <ClipboardIcon/>
          </button>
          <button
            type="button"
            class="btn btn-sm btn-icon-base h-fit"
            class:variant-ghost-warning={row.pinned}
            class:variant-soft={!row.pinned}
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
