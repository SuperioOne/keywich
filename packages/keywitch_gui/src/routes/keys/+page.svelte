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
    QrIcon
  } from "$lib/icons";
  import type {ModalActionResult} from "./types";
  import type {PageData} from "./$types";
  import {ModalAction} from "./types";
  import {RPC} from "$lib";
  import type {KeyMetadataItem} from "$lib";
  import {default as KeyForm} from "./create_key_form.svelte"
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
        response: (r: ModalActionResult) => resolve(r)
      });
    });

    if (response.type === ModalAction.submit) {
      await RPC.instance.add_key({
        charset: response.data.get("charset"),
        domain: response.data.get("domain"),
        custom_icon: response.data.get("custom_icon"),
        notes: response.data.get("notes"),
        revision: response.data.get("revision") ?? 0,
        tags: response.data.getAll("tags"),
        target_size: response.data.get("target_size"),
        user_name: response.data.get("username")
      });

      await invalidateAll();

      toastStore.trigger({
        message: "New key created successfully.",
        background: "variant-filled-success",
        timeout: 3000
      });
    }
  }

  function quick_copy(id: number, event) {
    console.debug(event);
  }

  async function advanced_copy(id: KeyMetadataItem, event) {
    console.debug(event);
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
      const success = await RPC.instance.remove_key(id);
      if (success) {
        await invalidateAll();
        toastStore.trigger({
          message: "Key removed from store",
          background: "variant-filled-warning",
          timeout: 3000
        });
      }
    }
  }
</script>

<div class="card p-5 shadow-xl" data-popup="copyPopupMenu">
  <div class="grid grid-cols-2 gap-3">
    <button type="button"
            class="btn p-1 w-16 h-16 variant-ghost-tertiary flex flex-col gap-1 justify-center align-middle">
      <TypeIcon/>
      <span class="font-mono text-xs text-center w-full !m-0">TEXT</span>
    </button>
    <button type="button"
            class="btn p-1 w-16 h-16 variant-ghost-tertiary flex flex-col gap-1 justify-center align-middle">
      <QrIcon/>
      <span class="font-mono text-xs text-center w-full !m-0">QR</span>
    </button>
    <button type="button"
            class="btn p-1 w-16 h-16 variant-ghost-tertiary flex flex-col gap-1 justify-center align-middle">
      <CodeIcon/>
      <span class="font-mono text-xs text-center w-full !m-0">JSON</span>
    </button>
    <button type="button"
            class="btn p-1 w-16 h-16 variant-ghost-tertiary flex flex-col gap-1 justify-center align-middle">
      <TerminalIcon/>
      <span class="font-mono text-xs text-center w-full !m-0">BASE64</span>
    </button>

  </div>
  <div class="arrow bg-surface-100-800-token border-1 border-amber-50"/>
</div>

<div class="flex gap-6 flex-col">
  <div class="flex flex-row gap-1">
    <button on:click={new_key} type="button" class="btn variant-filled w-full sm:w-auto"
    >
      <PlusCircleIcon/>
      <span class="font-bold"> Create </span>
    </button>
  </div>

  <dl class="flex flex-col gap-1">
    {#each data.keys as row,index (row.id)}
      <div
        role="none"
        class="card grid grid-cols-4 gap-3 w-full p-4">
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
            <div class="flex flex-wrap gap-1">
                <span
                  class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
                >
                  Preset
                </span>
              <span
                class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
              >
                  tag 1
                </span>
              <span
                class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
              >
                  tag 2
                </span>
              <span
                class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
              >
                  tag 3
                </span>
            </div>
          </div>
        </div>
        <div
          class="flex flex-col md:flex-row flex-wrap gap-1 sm:gap-2 justify-end items-center"
        >
          <button
            type="button"
            on:contextmenu|preventDefault={(ev) => advanced_copy(row, ev)}
            use:popup={{
              event: 'click',
              target: 'copyPopupMenu',
              placement: 'bottom',
            }}
            on:auxclick={quick_copy}
            on:click={quick_copy}
            class="btn btn-sm variant-soft-primary btn-icon-base h-fit"
          >
            <ClipboardIcon/>
          </button>
          <button
            type="button"
            class="btn btn-sm  btn-icon-base h-fit"
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
