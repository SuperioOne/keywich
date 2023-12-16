<script lang="ts">
  import RPC from "@keywitch/memory_rpc";
  import UploadIcon from "$lib/icons/upload.svelte";
  import type {ModalActionResult} from "./types";
  import type {PropertyError, KeyOptions, CharsetItem, KeyMetadataItem} from "@keywitch/rpc";
  import {InputChip, RangeSlider, getModalStore, FileDropzone} from "@skeletonlabs/skeleton";
  import {Log} from "$lib/logger";
  import {ModalAction} from "./types";
  import {getExtendedToastStore, i18nStore} from "$lib/stores";
  import {onDestroy, onMount} from "svelte";
  import {or_default} from "$lib/utils";

  export let data: KeyMetadataItem | undefined = undefined;

  let charsetList: CharsetItem[] = [];
  let errors: PropertyError<KeyOptions> = {};
  let formElement: HTMLFormElement;
  let icon: File | null = null;
  let iconUrl: string | null = data?.custom_icon ?? null;
  let noteValue: string | null = data?.notes ?? null;
  let sliderValue: number = data?.target_size ?? 32;
  let tags: string[] = data?.tags ?? [];

  const modalStore = getModalStore();
  const toastStore = getExtendedToastStore();
  const maximumNoteLength: number = 200;
  const maximumPassLength: number = 64;

  onMount(async () => {
    const charsetResult = await RPC.Charset.get_charsets();
    if (charsetResult.success) {
      charsetList = charsetResult.data;
    } else {
      Log.error(charsetResult.error);
      toastStore.trigger_error(i18nStore.getKey("i18:/key-form/errors/charset-error", "Unable to load charset list."));
    }
  });

  onDestroy(() => revoke_local_image());

  function revoke_local_image() {
    if (iconUrl && iconUrl !== data?.custom_icon) {
      URL.revokeObjectURL(iconUrl);
    }

    if (iconUrl && iconUrl === data?.custom_icon) {
      Log.debug(`Url is provided by data: ${iconUrl}`);
    }
  }

  function on_popup_close() {
    const modalInstance = $modalStore[0];
    if (!modalInstance) {
      Log.error(new Error("Close action failed. Modal component is created but unable to access modal itself."));
      return;
    }

    const modalResult: ModalActionResult = {
      type: ModalAction.closed
    }
    modalInstance.response?.(modalResult);
    modalStore.close();
  }

  function form_to_object(form: FormData) {
    const domain = form.get("domain");
    const user_name = form.get("user_name");
    const charset = form.get("charset");
    const target_size = or_default(Number(form.get("target_size")), 32);
    const notes = form.get("notes");
    const revision = or_default(Number(form.get("revision")), 1);
    const tags = form.getAll("tags");
    const custom_icon = form.get("custom_icon");

    return {
      domain: domain,
      charset: charset,
      custom_icon: custom_icon,
      notes: notes,
      revision: revision,
      tags: tags,
      target_size: target_size,
      user_name: user_name
    }
  }

  async function on_submit(event: Event) {
    const modalInstance = $modalStore[0];
    if (!modalInstance) {
      Log.error(new Error("Submit failed. Modal component is created but unable to access modal itself."));
      return;
    }

    if (!formElement) {
      Log.error(new Error("Key form ref is empty."));
      return;
    }

    const formData = new FormData(formElement);
    const keyData = form_to_object(formData) as unknown as KeyOptions;

    const result = data && !isNaN(Number(data.id))
      ? await RPC.KeyMetadata.update_key(data.id, keyData)
      : await RPC.KeyMetadata.add_key(keyData);

    if (result.success) {
      const modalResult: ModalActionResult = {
        type: ModalAction.submitted,
        data: result.data
      }
      modalInstance.response?.(modalResult);
      modalStore.close();
    } else {
      if (typeof result.error === "string") {
        Log.error(result.error);
        toastStore.trigger_error(result.error);
      } else {
        errors = result.error;
      }
    }
  }

  function on_custom_icon(event: Event) {
    const inputElement = event.target as HTMLInputElement;

    revoke_local_image();

    if (inputElement.files && inputElement.files.length > 0) {
      icon = inputElement.files.item(0);
      if (icon) {
        iconUrl = URL.createObjectURL(icon);
      }
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
      id="new_key_form"
      class="flex gap-5 flex-col"
      on:submit|preventDefault={on_submit}
    >
      <div>
        <label class="label">
          <span class="font-bold">{i18nStore.getKey("i18:/key-form/labels/domain", "Domain")}</span>
          <input
            class:input-error={errors.domain}
            class="input"
            name="domain"
            type="text"
            placeholder={i18nStore.getKey("i18:/key-form/desc/domain", "")}
            required
            value={data?.domain ?? null}
          />
        </label>
        {#if errors.domain}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.domain as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold">{i18nStore.getKey("i18:/key-form/labels/username", "Username")}</span>
          <input
            class:input-error={errors.user_name}
            class="input"
            type="text"
            name="user_name"
            placeholder={i18nStore.getKey("i18:/key-form/desc/username", "")}
            required
            value={data?.user_name ?? null}
          />
        </label>
        {#if errors.user_name}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.user_name as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label" for="charset">
          <span class="font-bold">{i18nStore.getKey("i18:/key-form/labels/charset", "Charset")}</span>
          <select
            class:input-error={errors.charset}
            class="select"
            name="charset"
            required
          >
            {#each charsetList as charsetItem (charsetItem.name)}
              <option
                selected="{data?.charset === charsetItem.charset}"
                value={charsetItem.charset}
              >
                {charsetItem.name}
              </option>
            {/each}
          </select>
        </label>
        {#if errors.charset}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.charset as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label" for="target_size">
          <span class="font-bold">{i18nStore.getKey("i18:/key-form/labels/pass-length", "Password Length")}</span>
          <RangeSlider
            name="target_size"
            bind:value={sliderValue}
            max={maximumPassLength}
            step={1}
            min={1}
          />
        </label>
        <div class="flex justify-end items-center">
          <div class=" text-xs">{sliderValue} / {maximumPassLength}</div>
        </div>
        {#if errors.target_size}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.target_size as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label" for="tags">
          <span class="font-bold">{i18nStore.getKey("i18:/key-form/labels/tags", "Tags")}</span>
          <InputChip
            bind:value={tags}
            name="tags"
            chips="variant-filled-primary"
            placeholder={i18nStore.getKey("i18:/key-form/desc/tags", "")}
          />
        </label>
        {#if errors.tags}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.tags as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold">{i18nStore.getKey("i18:/key-form/labels/note", "Note")}</span>
          <textarea
            class:input-error={errors.notes}
            class="textarea"
            rows="4"
            name="notes"
            placeholder={i18nStore.getKey("i18:/key-form/desc/note", "")}
            maxlength={maximumNoteLength}
            bind:value={noteValue}
          />
        </label>
        <div class="flex justify-end items-center">
          <div class="text-xs">{noteValue?.length ?? 0} / {maximumNoteLength}</div>
        </div>
        {#if errors.notes}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.notes as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold">{i18nStore.getKey("i18:/key-form/labels/revision", "Revision No")}</span>
          <input
            class:input-error={errors.revision}
            class="input"
            type="number"
            name="revision"
            min="0"
            step="1"
            placeholder={i18nStore.getKey("i18:/key-form/desc/revision", "")}
            value={data?.revision ?? 0}
          />
        </label>
        {#if errors.revision}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.revision as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label" for="custom_icon">
          <span class="font-bold">{i18nStore.getKey("i18:/key-form/labels/icon", "Custom Icon")}</span>
          <FileDropzone
            accept="image/png, image/jpeg, image/svg+xml"
            on:change={on_custom_icon}
            name="custom_icon"
          >
            <svelte:fragment slot="lead">
              <div class="flex flex-row justify-center items-center">
                {#if iconUrl}
                  <img width="64px" src={iconUrl} alt="image"/>
                {:else }
                  <UploadIcon/>
                {/if}
              </div>
            </svelte:fragment>
            <svelte:fragment slot="message">
              <p>
                {i18nStore.getKey("i18:/key-form/desc/icon", "Upload a image or drag and drop")}
              </p>
            </svelte:fragment>
          </FileDropzone>
        </label>
        {#if errors.custom_icon}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.custom_icon as error}
              <li>{error}</li>
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
        <span>{i18nStore.getKey("i18:/generic/cancel", "Cancel")}</span>
      </button>

      <button
        type="button"
        class="btn variant-soft-primary"
        on:click={on_submit}
      >
        <span>{i18nStore.getKey("i18:/generic/confirm", "Confirm")}</span>
      </button>
    </div>
  </div>
{/if}