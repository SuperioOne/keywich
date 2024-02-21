<script lang="ts">
  import FilterIcon from "$lib/icons/filter.svelte";
  import PlusCircleIcon from "$lib/icons/plus-circle.svelte";
  import type {KeyItem} from "@keywich/api";
  import type {PageData} from "./$types";
  import type {ModalActionResult, TokenType} from "$lib";
  import {
    KeyRow, KeyFilterInput, i18nStore, getToastStore, KeyForm, KeyUpdateForm, AdvancedCopyMenu, Log, ModalAction, RPC
  } from "$lib";
  import {fly} from "svelte/transition";
  import {goto, invalidateAll} from "$app/navigation";
  import {getModalStore} from "@skeletonlabs/skeleton";
  import {is_error_response} from "@keywich/api/utils";

  export let data: PageData;

  let selected: number | undefined = undefined;
  const modal_store = getModalStore();
  const toast_store = getToastStore();

  async function create_key() {
    const modal_response = await new Promise<ModalActionResult<void>>((resolve) => {
      modal_store.trigger({
        component: {
          ref: KeyForm,
        },
        title: $i18nStore.get_key("i18:/actions/create-key/title", "Create New Key"),
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult<void>) => resolve(r),
      });
    });

    if (modal_response?.type === ModalAction.submitted) {
      toast_store.trigger_success($i18nStore.get_key(`i18:/actions/create-key/msg/success`, "New key created."));
      await invalidateAll();
    }
  }

  async function update_key(event: CustomEvent<KeyItem>) {
    const modal_response = await new Promise<ModalActionResult<void>>((resolve) => {
      modal_store.trigger({
        component: {
          ref: KeyUpdateForm,
          props: {
            data: event.detail
          }
        },
        title: $i18nStore.get_key("i18:/actions/update-key/title", "Update Key"),
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult<void>) => resolve(r),
      });
    });

    if (modal_response?.type === ModalAction.submitted) {
      toast_store.trigger_success($i18nStore.get_key(`i18:/actions/update-key/msg/success`, "Key updated successfully."));
      await invalidateAll();
    }
  }

  async function flip_pin(event: CustomEvent<KeyItem>) {
    const rpcAction = event.detail.pinned ? RPC.unpin_key : RPC.pin_key;

    try {
      await rpcAction(event.detail.id);
      await invalidateAll();
    } catch (err) {
      Log.error(err);
      toast_store.trigger_error($i18nStore.get_key(`i18:/actions/pin-key/msg/error`, "Unable to pin"));

      if (is_error_response(err)) {
        toast_store.trigger_error($i18nStore.get_key(`i18:/errors/${err.code}`, err.message));
      }
    }
  }

  async function quick_copy(event: CustomEvent<KeyItem>) {
    try {
      const result = await RPC.generate_password_from({
        content: "test",
        profile_id: event.detail.id,
        output_type: "Text"
      });

      await RPC.copy_to_clipboard(result);
      toast_store.trigger_success($i18nStore.get_key(`i18:/actions/copy-key/msg/success`, "Key copied."));
    } catch (err) {
      Log.error(err);
      toast_store.trigger_error($i18nStore.get_key(`i18:/actions/copy-key/msg/error`, "Key generation failed."));

      if (is_error_response(err)) {
        toast_store.trigger_error($i18nStore.get_key(`i18:/errors/${err.code}`, err.message));
      }
    }
  }

  async function advanced_copy(event: CustomEvent<KeyItem>) {
    try {
      await new Promise<ModalActionResult<KeyItem>>((resolve) => {
        modal_store.trigger({
          component: {
            ref: AdvancedCopyMenu,
            props: {
              keyId: event.detail.id
            }
          },
          title: $i18nStore.get_key(`i18:/actions/advanced-copy/title`, "Advanced"),
          backdropClasses: "backdrop-blur-sm",
          type: "component",
          response: (r: ModalActionResult<KeyItem>) => resolve(r),
        });
      });
    } catch (err) {
      Log.error(err);

      if (is_error_response(err)) {
        toast_store.trigger_error($i18nStore.get_key(`i18:/errors/${err.code}`, err.message));
      }
    }
  }

  async function delete_key(event: CustomEvent<KeyItem>) {
    const confirmation = await new Promise((resolve) => {
      modal_store.trigger({
        type: "confirm",
        title: $i18nStore.get_key("i18:/actions/delete-key/title", "Confirm Action"),
        body: $i18nStore.get_key(
          `i18:/actions/delete-key/message?$noCache&username=${event.detail.username}&domain=${event.detail.domain}`,
          "Are you sure to delete key?"
        ),
        buttonTextConfirm: $i18nStore.get_key("i18:/generic/delete", "Delete"),
        buttonTextCancel: $i18nStore.get_key("i18:/generic/cancel", "Cancel"),
        response: (r: boolean) => resolve(r),
      });
    });

    if (confirmation) {
      try {
        await RPC.delete_key(event.detail.id);
        toast_store.trigger_warning($i18nStore.get_key("i18:/actions/delete-key/msg/success", "Key deleted from store."));
        await invalidateAll();
      } catch (err) {
        Log.warn(err);
        toast_store.trigger_error($i18nStore.get_key("i18:/actions/delete-key/msg/error", "Unable to delete key."));

        if (is_error_response(err)) {
          toast_store.trigger_error($i18nStore.get_key(`i18:/errors/${err.code}`, err.message));
        }
      }
    }
  }

  async function _search(tokens: TokenType[]) {
    const target = new URL("/keys", document.location.origin);

    if (tokens && tokens.length > 0) {
      let name: string;

      for (const searchQuery of tokens) {
        switch (searchQuery.type) {
          case "username":
            name = "u";
            break;
          case "domain":
            name = "d";
            break;
          case "tag":
            name = "t";
            break;
          case "term":
            name = "s";
            break;
        }

        target.searchParams.append(name, searchQuery.value);
      }
    }

    await goto(target, {
      invalidateAll: true,
      keepFocus: true,
    });
  }

  async function search_keys(event: CustomEvent<TokenType[]>) {
    await _search(event.detail ?? []);
  }

  async function on_tag(event: CustomEvent<string>) {
    const searchTokens: TokenType[] = [{type: "tag", value: event.detail}];
    await _search(searchTokens);
  }
</script>

<div class="flex gap-6 flex-col">
  <div class="grid grid-cols-2 gap-6">
    <div class="col-span-full sm:col-span-1 flex flex-row flex-wrap gap-2">
      <button
          on:click={create_key}
          type="button"
          class="btn variant-filled-primary w-full sm:w-auto"
      >
        <PlusCircleIcon/>
        <span class="font-bold"> {$i18nStore.get_key("i18:/keys/button/create", "Create")} </span>
      </button>
    </div>
    <div class="col-span-full sm:col-span-1 flex flex-row flex-wrap gap-2 justify-end">
      <div class="w-full sm:w-fit">
        <KeyFilterInput
            on:search={search_keys}
            tokens={data.tokens ?? []}
        >
          <FilterIcon size={18}/>
          <span>
            {$i18nStore.get_key("i18:/keys/button/filter", "Filter")}
          </span>
        </KeyFilterInput>
      </div>
    </div>
  </div>

  {#if data.keys.length < 1}
    <p class="text-center w-full font-light text-xl py-6">
      {$i18nStore.get_key("i18:/keys/empty-list", "Empty list")}
    </p>
  {:else }
    <div class="flex flex-col gap-1">
      {#each data.keys as row,index (row.id)}
        <div class="w-full" transition:fly={{duration:200, y:20}}>
          <KeyRow
              item={row}
              active={index === selected}
              on:copy={quick_copy}
              on:copyAux={quick_copy}
              on:copyAlt={advanced_copy}
              on:delete={delete_key}
              on:update={update_key}
              on:pin={flip_pin}
              on:tagSelect={on_tag}
          />
        </div>
      {/each}
    </div>
  {/if}
</div>
