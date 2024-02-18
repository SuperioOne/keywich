<script lang="ts">
  import UploadIcon from "../../icons/upload.svelte";
  import AlertIcon from "../../icons/alert-triangle.svelte";
  import type {CharsetItem, CustomIconType, KeyItem, KeyRequest, KeyUpdateRequest, PropertyError} from "@keywich/api";
  import type {ModalActionResult} from "./types";
  import {ModalAction} from "./types";
  import {Accordion, AccordionItem, FileDropzone, getModalStore, InputChip, RangeSlider} from "@skeletonlabs/skeleton";
  import {Log} from "../../logger";
  import {RPC} from "../../rpc";
  import {getToastStore, i18nStore} from "../../stores";
  import {onDestroy, onMount} from "svelte";
  import {is_null_or_empty, or_default} from "@keywich/api/utils";

  export let data: KeyItem;

  const modal_store = getModalStore();
  const toast_store = getToastStore();
  const max_note_len: number = 200;
  const max_pass_len: number = 64;

  let charset_list: CharsetItem[] = [];
  let errors: PropertyError<KeyRequest> = {};
  let form_element: HTMLFormElement;
  let icon_file: File | string | null = data.custom_icon ?? null;
  let icon_url: string | undefined = data.custom_icon ? RPC.convert_icon_src(data.custom_icon) : undefined;
  let note_value: string | null = data.notes ?? null;
  let slider_value: number = data.target_size ?? 32;
  let tags: string[] = data.tags ?? [];

  onMount(async () => {
    try {
      charset_list = await RPC.get_charsets();
    } catch (err) {
      Log.error(err);
      toast_store.trigger_error($i18nStore.get_key("i18:/key-form/errors/charset-error", "Unable to load charset list."));
    }
  });

  onDestroy(() => {
    if (!is_null_or_empty(icon_url)) {
      URL.revokeObjectURL(icon_url);
      Log.debug(`Removed temporary object: ${icon_url}.`);
    }
  });

  function on_popup_close() {
    const modal_inst = $modal_store[0];
    if (!modal_inst) {
      Log.error("Close action failed. Modal component is created but unable to access modal itself.");
      return;
    }

    const modal_result: ModalActionResult<boolean> = {
      type: ModalAction.closed
    }
    modal_inst.response?.(modal_result);
    modal_store.close();
  }

  async function form_to_object(form: FormData): Promise<KeyUpdateRequest> {
    const domain = or_default(form.get("domain"), data.domain);
    const username = or_default(form.get("username"), data.username);
    const charset = or_default(form.get("charset"), data.charset);
    const target_size = or_default(parseInt(form.get("target_size")?.toString() ?? ""), data.target_size);
    const notes = form.get("notes") ?? data.notes;
    const revision = or_default(parseInt(form.get("revision")?.toString() ?? ""), data.revision);
    const tags = or_default(form.getAll("tags"), data.tags);

    let icon_value: CustomIconType | undefined;
    if (typeof icon_file === "string") {
      icon_value = {
        type: "name",
        name: icon_file
      };
    } else if (icon_file !== null) {
      const buffer = await icon_file.arrayBuffer()
      icon_value = {
        type: "buffer",
        data: new Uint8Array(buffer)
      };
    }

    return {
      domain: domain as string,
      charset: charset as string,
      notes: notes as string,
      revision: revision,
      tags: tags as string[],
      target_size: target_size,
      username: username as string,
      custom_icon: icon_value
    };
  }

  async function on_submit() {
    const modal_inst = $modal_store[0];
    if (!modal_inst) {
      Log.error(new Error("Submit failed. Modal component is created but unable to access modal itself."));
      return;
    }

    if (!form_element) {
      Log.error(new Error("Key form ref is empty."));
      return;
    }

    try {
      const form_data = new FormData(form_element);
      const update_req = await form_to_object(form_data);

      await RPC.update_key(data.id, update_req);

      const model_result: ModalActionResult<boolean> = {
        type: ModalAction.submitted,
        data: true
      }
      modal_inst.response?.(model_result);
      modal_store.close();
    } catch (err) {
      Log.error(err);
      toast_store.trigger_error("failed");
    }
  }

  function on_custom_icon(event: Event) {
    const input_element = event.target as HTMLInputElement;

    if (input_element.files && input_element.files.length > 0) {
      icon_file = input_element.files.item(0);

      if (!is_null_or_empty(icon_url)) {
        URL.revokeObjectURL(icon_url);
        Log.debug(`Removed temporary object: ${icon_url}.`);
      }

      if (icon_file && icon_file.size > 0) {
        icon_url = URL.createObjectURL(icon_file);
      }
    }
  }

  function on_clear_icon() {
    icon_file = null;
    icon_url = undefined;
  }
</script>

{#if $modal_store[0]}
  <div class="card px-16 py-8 w-full sm:w-modal flex flex-col gap-10">
    <h2 class="font-bold h2">
      {$modal_store[0].title}
    </h2>
    <form
        bind:this={form_element}
        id="new_key_form"
        class="flex gap-4 flex-col"
        on:submit|preventDefault={on_submit}
    >
      <div>
        <label class="label" for="tags">
          <span class="font-bold">{$i18nStore.get_key("i18:/key-form/labels/tags", "Tags")}</span>
          <InputChip
              bind:value={tags}
              name="tags"
              chips="variant-filled-primary"
              placeholder={$i18nStore.get_key("i18:/key-form/desc/tags", "")}
          />
        </label>
        {#if errors.tags}
          <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
            {#each errors.tags as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold">{$i18nStore.get_key("i18:/key-form/labels/note", "Note")}</span>
          <textarea
              class:input-error={errors.notes}
              class="textarea"
              rows="4"
              name="notes"
              placeholder={$i18nStore.get_key("i18:/key-form/desc/note", "")}
              maxlength={max_note_len}
              bind:value={note_value}
          />
        </label>
        <div class="flex justify-end items-center">
          <div class="text-xs">{note_value?.length ?? 0} / {max_note_len}</div>
        </div>
        {#if errors.notes}
          <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
            {#each errors.notes as error}
              <li> {error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label" for="custom_icon">
          <span class="font-bold">{$i18nStore.get_key("i18:/key-form/labels/icon", "Custom Icon")}</span>
          <FileDropzone
              accept="image/png, image/jpeg"
              on:change={on_custom_icon}
              name="custom_icon"
          >
            <svelte:fragment slot="lead">
              <div class="flex flex-row justify-center items-center">
                {#if icon_url}
                  <img width="64px" src={icon_url} alt="icon"/>
                {:else }
                  <UploadIcon/>
                {/if}
              </div>
            </svelte:fragment>
            <svelte:fragment slot="message">
              <p>
                {$i18nStore.get_key("i18:/key-form/desc/icon", "Upload a image or drag and drop")}
              </p>
            </svelte:fragment>
          </FileDropzone>
          <span class="flex flex-row justify-end py-2">
            {#if !is_null_or_empty(icon_url)}
              <button
                  type="button"
                  class="btn btn-sm variant-ghost-secondary"
                  on:click={on_clear_icon}
              >
                {$i18nStore.get_key("i18:/generic/clear", "Clear")}
              </button>
            {/if}
          </span>
        </label>
        {#if errors.custom_icon}
          <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
            {#each errors.custom_icon as error}
              <li>{error}</li>
            {/each}
          </ul>
        {/if}
      </div>

      <Accordion>
        <AccordionItem>
          <svelte:fragment slot="lead">
            <span class="text-warning-600">
            <AlertIcon size={18}/>
              </span>
          </svelte:fragment>
          <svelte:fragment slot="summary">
            {$i18nStore.get_key("i18:/key-form/advanced", "Advanced Options")}
          </svelte:fragment>
          <svelte:fragment slot="content">

            <aside class="alert variant-ghost-warning">
              <div>
                <AlertIcon size={22}/>
              </div>
              <div class="alert-message">
                <p>
                  {$i18nStore.get_key(
                    "i18:/key-form/advanced/warning-desc",
                    "Changing advanced options will change the calculated password.")}
                </p>
              </div>
            </aside>

            <div>
              <label class="label">
                <span class="font-bold">{$i18nStore.get_key("i18:/key-form/labels/domain", "Domain")}</span>
                <input
                    class:input-error={errors.domain}
                    class="input"
                    name="domain"
                    type="text"
                    placeholder={$i18nStore.get_key("i18:/key-form/desc/domain", "")}
                    required
                    value={data?.domain ?? null}
                />
              </label>
              {#if errors.domain}
                <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
                  {#each errors.domain as error}
                    <li> {error}</li>
                  {/each}
                </ul>
              {/if}
            </div>

            <div>
              <label class="label">
                <span class="font-bold">{$i18nStore.get_key("i18:/key-form/labels/username", "Username")}</span>
                <input
                    class:input-error={errors.username}
                    class="input"
                    type="text"
                    name="username"
                    placeholder={$i18nStore.get_key("i18:/key-form/desc/username", "")}
                    required
                    value={data?.username ?? null}
                />
              </label>
              {#if errors.username}
                <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
                  {#each errors.username as error}
                    <li>{error}</li>
                  {/each}
                </ul>
              {/if}
            </div>

            <div>
              <label class="label" for="charset">
                <span class="font-bold">{$i18nStore.get_key("i18:/key-form/labels/charset", "Charset")}</span>
                <select
                    class:input-error={errors.charset}
                    class="select"
                    name="charset"
                    required
                >
                  {#each charset_list as charsetItem (charsetItem.name)}
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
                <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
                  {#each errors.charset as error}
                    <li> {error}</li>
                  {/each}
                </ul>
              {/if}
            </div>

            <div>
              <label class="label" for="target_size">
                <span
                    class="font-bold">{$i18nStore.get_key("i18:/key-form/labels/pass-length", "Password Length")}</span>
                <RangeSlider
                    name="target_size"
                    bind:value={slider_value}
                    max={max_pass_len}
                    step={1}
                    min={1}
                />
              </label>
              <div class="flex justify-end items-center">
                <div class=" text-xs">{slider_value} / {max_pass_len}</div>
              </div>
              {#if errors.target_size}
                <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
                  {#each errors.target_size as error}
                    <li> {error}</li>
                  {/each}
                </ul>
              {/if}
            </div>

            <div>
              <label class="label">
                <span class="font-bold">{$i18nStore.get_key("i18:/key-form/labels/revision", "Revision No")}</span>
                <input
                    class:input-error={errors.revision}
                    class="input"
                    type="number"
                    name="revision"
                    min="0"
                    step="1"
                    placeholder={$i18nStore.get_key("i18:/key-form/desc/revision", "")}
                    value={data?.revision ?? 0}
                />
              </label>
              {#if errors.revision}
                <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
                  {#each errors.revision as error}
                    <li>{error}</li>
                  {/each}
                </ul>
              {/if}
            </div>
          </svelte:fragment>
        </AccordionItem>
      </Accordion>
    </form>
    <div class="flex flex-row justify-between">
      <button
          type="button"
          class="btn variant-soft"
          on:click|preventDefault={on_popup_close}
      >
        <span>{$i18nStore.get_key("i18:/generic/cancel", "Cancel")}</span>
      </button>

      <button
          type="button"
          class="btn variant-filled-primary"
          on:click={on_submit}
      >
        <span>{$i18nStore.get_key("i18:/generic/confirm", "Confirm")}</span>
      </button>
    </div>
  </div>
{/if}