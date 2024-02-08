<script lang="ts">
  import type {ModalActionResult} from "./types";
  import type {PropertyError, CharsetOptions, CharsetItem} from "@keywich/api";
  import {getModalStore} from "@skeletonlabs/skeleton";
  import {Log} from "../../logger";
  import {ModalAction} from "./types";
  import {RPC} from "../../rpc";
  import {getExtendedToastStore, i18nStore} from "../../stores";

  let errors: PropertyError<CharsetOptions> = {};
  let formElement: HTMLFormElement;

  const modalStore = getModalStore();
  const toastStore = getExtendedToastStore();

  function on_popup_close() {
    const modalInstance = $modalStore[0];
    if (!modalInstance) {
      Log.error(new Error("Close action failed. Modal component is created but unable to access modal itself."));
      return;
    }

    const modalResult: ModalActionResult<CharsetItem> = {
      type: ModalAction.closed
    }
    modalInstance.response?.(modalResult);
    modalStore.close();
  }

  function form_to_object(form: FormData) {
    const charset = form.get("charset");
    const name = form.get("name");
    const description = form.get("description");

    return {
      charset,
      name,
      description
    }
  }

  async function on_submit() {
    const modalInstance = $modalStore[0];
    if (!modalInstance) {
      Log.error(new Error("Submit failed. Modal component is created but unable to access modal itself."));
      return;
    }

    if (!formElement) {
      Log.error(new Error("Charset form ref is empty."));
      return;
    }

    const formData = new FormData(formElement);
    const charsetData = form_to_object(formData) as unknown as CharsetOptions;

    try {
      let result = await RPC.insert_charset(charsetData);

      const modalResult: ModalActionResult<string> = {
        type: ModalAction.submitted,
        data: result
      }

      modalInstance.response?.(modalResult);
      modalStore.close();
    } catch (err) {
      Log.error(err);
      toastStore.trigger_error(err as string);

      // TODO: get validation errors
      // errors = result.error;
    }
  }
</script>

{#if $modalStore[0]}
  <div class="card px-16 py-8 w-full sm:w-modal flex flex-col gap-10">
    <h2 class="font-bold h2">
      {$modalStore[0].title}
    </h2>
    <form
        bind:this={formElement}
        id="charset_form"
        class="flex gap-5 flex-col"
        on:submit|preventDefault={on_submit}
    >
      <div>
        <label class="label">
          <span class="font-bold">{i18nStore.get_key("i18:/charset-form/labels/name", "Name")}</span>
          <input
              class:input-error={errors.name}
              class="input"
              name="name"
              type="text"
              placeholder={i18nStore.get_key("i18:/charset-form/desc/name", "")}
              required
          />
        </label>
        {#if errors.name}
          <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
            {#each errors.name as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold">{i18nStore.get_key("i18:/charset-form/labels/charset", "Charset")}</span>
          <input
              class:input-error={errors.charset}
              class="input"
              type="text"
              name="charset"
              placeholder={i18nStore.get_key("i18:/charset-form/desc/charset", "")}
              required
          />
        </label>
        {#if errors.charset}
          <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
            {#each errors.charset as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold">{i18nStore.get_key("i18:/charset-form/labels/description", "Description")}</span>
          <input
              class:input-error={errors.description}
              class="input"
              type="text"
              name="description"
              placeholder={i18nStore.get_key("i18:/charset-form/desc/description", "")}
          />
        </label>
        {#if errors.description}
          <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
            {#each errors.description as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

    </form>
    <div class="flex flex-row justify-between">
      <button
          type="button"
          class="btn variant-soft"
          on:click|preventDefault={on_popup_close}
      >
        <span>{i18nStore.get_key("i18:/generic/cancel", "Cancel")}</span>
      </button>

      <button
          type="button"
          class="btn variant-filled-primary"
          on:click={on_submit}
      >
        <span>{i18nStore.get_key("i18:/generic/confirm", "Confirm")}</span>
      </button>
    </div>
  </div>
{/if}