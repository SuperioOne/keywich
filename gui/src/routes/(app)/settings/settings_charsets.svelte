<script lang="ts">
  import PlusCircleIcon from "$lib/icons/plus-circle.svelte";
  import TrashIcon from "$lib/icons/trash-2.svelte";
  import type {CharsetItem} from "@keywich/api";
  import {CharsetForm, getToastStore, i18nStore, Log, ModalAction, type ModalActionResult, RPC} from "$lib";
  import {fly} from "svelte/transition";
  import {invalidateAll} from "$app/navigation";
  import {getModalStore} from "@skeletonlabs/skeleton";
  import {is_error_response} from "@keywich/api/utils";

  export let charsets: CharsetItem[];

  const modal_store = getModalStore();
  const toast_store = getToastStore();

  async function delete_charset(charset: CharsetItem) {
    const confirmation = await new Promise<boolean>((resolve) => {
      modal_store.trigger({
        type: "confirm",
        title: $i18nStore.get_key("i18:/actions/delete-charset/title", "Confirm Action"),
        body: $i18nStore.get_key(
          `i18:/actions/delete-charset/message?$noCache&name=${charset.name}`,
          "Are you sure to delete charset?"
        ),
        buttonTextConfirm: $i18nStore.get_key("i18:/generic/delete", "Delete"),
        buttonTextCancel: $i18nStore.get_key("i18:/generic/cancel", "Cancel"),
        response: (r: boolean) => resolve(r),
      });
    });

    if (confirmation) {
      try {
        await RPC.delete_charset(charset.name);

        toast_store.trigger_warning($i18nStore.get_key(
          "i18:/actions/delete-charset/msg/success",
          "Charset deleted."));

        await invalidateAll();
      } catch (err) {
        Log.warn(err);

        toast_store.trigger_error($i18nStore.get_key(
          "i18:/actions/delete-charset/msg/error",
          "Unable to delete charset."));

        if (is_error_response(err)) {
          toast_store.trigger_error($i18nStore.get_key(`i18:/errors/${err.code}`, err.message));
        }
      }
    }
  }

  async function create_charset() {
    const response = await new Promise<ModalActionResult<string>>((resolve) => {
      modal_store.trigger({
        component: {
          ref: CharsetForm,
        },
        title: $i18nStore.get_key("i18:/actions/create-charset/title", "New Charset"),
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult<string>) => resolve(r),
      });
    });

    if (response?.type === ModalAction.submitted) {
      toast_store.trigger_success($i18nStore.get_key(
        `i18:/actions/create-charset/msg/success?$noCache&name=${response}`,
        "New charset created."));

      await invalidateAll();
    }
  }
</script>

<div class="flex flex-col gap-2">
  <div>
    <button
        on:click={create_charset}
        type="button"
        class="btn variant-filled-primary w-full sm:w-auto"
    >
      <PlusCircleIcon/>
      <span class="font-bold"> {$i18nStore.get_key("i18:/settings/charsets/create", "Create")} </span>
    </button>
  </div>
  <ul>
    {#if charsets.length < 1 }
      <li class="text-center w-full text-xl font-light">
        {$i18nStore.get_key("i18:/settings/charsets/empty-list", "Empty charset list")}
      </li>
    {:else}
      {#each charsets as charset (charset.name)}
        <li class="py-4 flex flex-row justify-between items-center" transition:fly={{duration:200, y:20}}>
          <dl>
            <dt>
              <p class="text-secondary-400-500-token">
                <strong class="text-lg">
                  {charset.name}
                </strong>
              </p>
              <p class="text-tertiary-400-500-token">
                <small>
                  [{charset.charset}]
                </small>
              </p>
            </dt>
            <dd class="font-light">
              <small>
                {charset.description ?? ""}
              </small>
            </dd>
          </dl>
          <button
              class="btn btn-sm variant-glass-error btn-icon-base h-fit"
              on:click={() => delete_charset(charset)}
          >
            <TrashIcon size={23}/>
          </button>
        </li>
        <hr>
      {/each}
    {/if}
  </ul>

</div>