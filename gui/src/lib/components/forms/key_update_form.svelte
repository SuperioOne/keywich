<script lang="ts">
  import AlertIcon from "../../icons/alert-triangle.svelte";
  import type {
    CharsetItem,
    CustomIconType,
    KeyItem,
    KeyRequest,
    KeyUpdateRequest,
  } from "../../api";
  import type { ModalActionResult } from "./types";
  import { ModalAction } from "./types";
  import {
    getModalStore,
    InputChip,
    ProgressRadial,
    RangeSlider,
  } from "@skeletonlabs/skeleton";
  import { Log } from "../../logger";
  import { Api } from "../../api";
  import type { ValidationError } from "../../utils";
  import { getToastStore, i18nStore } from "../../stores";
  import { onDestroy, onMount } from "svelte";
  import {
    is_error_response,
    is_null_or_empty,
    is_validation_error_response,
    or_default,
  } from "../../utils";

  export let data: KeyItem;

  const modal_store = getModalStore();
  const toast_store = getToastStore();
  const max_note_len: number = 200;
  const max_pass_len: number = 64;

  let charset_list: CharsetItem[] = [];
  let field_errors: ValidationError<KeyRequest> = {};
  let form_element: HTMLFormElement;
  let icon_element: HTMLInputElement;
  let selected_charset: string = data.charset;
  let icon_file: File | string | null = data.custom_icon ?? null;
  let icon_url: string | undefined = data.custom_icon
    ? Api.convert_icon_src(data.custom_icon)
    : undefined;
  let note_value: string | null = data.notes ?? null;
  let slider_value: number = data.target_size ?? 32;
  let tags: string[] = data.tags ?? [];
  let submitting: boolean = false;

  onMount(async () => {
    try {
      charset_list = await Api.get_charsets();
      if (charset_list.findIndex((e) => e.charset === data.charset) < 0) {
        charset_list.push({
          charset: data.charset,
          name: "Unknown",
        });
      }
    } catch (err) {
      Log.error(err);
      toast_store.trigger_error(
        $i18nStore.get_key(
          "i18:/key-form/errors/charset-error",
          "Unable to load charset list.",
        ),
      );

      if (is_error_response(err)) {
        toast_store.trigger_error(
          $i18nStore.get_key(`i18:/errors/${err.code}`, err.message),
        );
      }
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
      Log.error(
        "Close action failed. Modal component is created but unable to access modal itself.",
      );
      return;
    }

    const modal_result: ModalActionResult<boolean> = {
      type: ModalAction.closed,
    };
    modal_inst.response?.(modal_result);
    modal_store.close();
  }

  async function form_to_object(form: FormData): Promise<KeyUpdateRequest> {
    const domain = or_default(form.get("domain"), data.domain);
    const username = or_default(form.get("username"), data.username);
    const charset = or_default(form.get("charset"), data.charset);
    const target_size = or_default(
      parseInt(form.get("target_size")?.toString() ?? ""),
      data.target_size,
    );
    const notes = form.get("notes") ?? data.notes;
    const revision = or_default(
      parseInt(form.get("revision")?.toString() ?? ""),
      data.revision,
    );
    const tags = or_default(form.getAll("tags"), data.tags);

    let icon_value: CustomIconType | undefined;
    if (typeof icon_file === "string") {
      icon_value = {
        type: "name",
        name: icon_file,
      };
    } else if (icon_file !== null) {
      icon_file.webkitRelativePath;
      const buffer = await icon_file.arrayBuffer();
      icon_value = {
        type: "buffer",
        data: new Uint8Array(buffer),
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
      custom_icon: icon_value,
    };
  }

  async function on_submit() {
    const modal_inst = $modal_store[0];
    if (!modal_inst) {
      Log.error(
        new Error(
          "Submit failed. Modal component is created but unable to access modal itself.",
        ),
      );
      return;
    }

    if (!form_element) {
      Log.error(new Error("Key form ref is empty."));
      return;
    }

    if (!form_element.reportValidity()) {
      return;
    }

    try {
      const form_data = new FormData(form_element);
      const update_req = await form_to_object(form_data);

      await Api.update_key(data.id, update_req);

      const model_result: ModalActionResult<boolean> = {
        type: ModalAction.submitted,
        data: true,
      };
      modal_inst.response?.(model_result);
      modal_store.close();
    } catch (err) {
      Log.error(err);

      if (is_error_response(err)) {
        toast_store.trigger_error(
          $i18nStore.get_key(`i18:/errors/${err.code}`, err.message),
        );

        if (is_validation_error_response(err)) {
          field_errors = err.fields;
        }
      } else {
        toast_store.trigger_error(
          $i18nStore.get_key(
            "i18:/key-form/errors/unknown-update-error",
            "Key update failed.",
          ),
        );
      }
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
    icon_element.files = null;
    icon_element.value = "";
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
    >
      <div>
        <label class="label" for="tags">
          <span class="font-bold"
            >{$i18nStore.get_key("i18:/key-form/labels/tags", "Tags")}</span
          >
          <InputChip
            bind:value={tags}
            name="tags"
            chips="variant-filled-primary"
            placeholder={$i18nStore.get_key("i18:/key-form/desc/tags", "")}
          />
        </label>
        {#if field_errors.tags}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.tags as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}`,
                  error.message ?? "",
                )}
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold"
            >{$i18nStore.get_key("i18:/key-form/labels/note", "Note")}</span
          >
          <textarea
            class:input-error={field_errors.notes}
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
        {#if field_errors.notes}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.notes as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}`,
                  error.message ?? "",
                )}
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label" for="custom_icon">
          <span class="font-bold">
            {$i18nStore.get_key("i18:/key-form/labels/icon", "Custom Icon")}
          </span>
          <input
            bind:this={icon_element}
            class="input"
            type="file"
            name="custom_icon"
            accept="image/png, image/jpeg"
            on:change={on_custom_icon}
            placeholder={$i18nStore.get_key(
              "i18:/key-form/desc/icon",
              "Upload a image or drag and drop",
            )}
          />
          {#if icon_url}
            <span class="font-bold inline-block py-2">
              {$i18nStore.get_key("i18:/generic/preview", "Preview")}
            </span>
            <span class="card p-3 flex flex-col justify-center items-center">
              <img width="128px" src={icon_url} alt="icon" />
            </span>
            <span class="flex flex-row justify-end py-2">
              <button
                type="button"
                class="btn btn-sm variant-ghost-secondary"
                on:click={on_clear_icon}
              >
                {$i18nStore.get_key("i18:/generic/clear", "Clear")}
              </button>
            </span>
          {/if}
        </label>
        {#if field_errors.custom_icon}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.custom_icon as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}`,
                  error.message ?? "",
                )}
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      <h2 class="font-bold text-xl">
        {$i18nStore.get_key("i18:/key-form/advanced", "Advanced Options")}
      </h2>

      <aside class="alert variant-ghost-warning">
        <div>
          <AlertIcon size={22} />
        </div>
        <div class="alert-message">
          <p>
            {$i18nStore.get_key(
              "i18:/key-form/advanced/warning-desc",
              "Changing advanced options will change the calculated password.",
            )}
          </p>
        </div>
      </aside>

      <div>
        <label class="label">
          <span class="font-bold"
            >{$i18nStore.get_key("i18:/key-form/labels/domain", "Domain")}</span
          >
          <input
            class:input-error={field_errors.domain}
            class="input"
            name="domain"
            type="text"
            placeholder={$i18nStore.get_key("i18:/key-form/desc/domain", "")}
            required
            value={data?.domain ?? null}
          />
        </label>
        {#if field_errors.domain}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.domain as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}`,
                  error.message ?? "",
                )}
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold"
            >{$i18nStore.get_key(
              "i18:/key-form/labels/username",
              "Username",
            )}</span
          >
          <input
            class:input-error={field_errors.username}
            class="input"
            type="text"
            name="username"
            placeholder={$i18nStore.get_key("i18:/key-form/desc/username", "")}
            required
            value={data?.username ?? null}
          />
        </label>
        {#if field_errors.username}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.username as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}`,
                  error.message ?? "",
                )}
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label" for="charset">
          <span class="font-bold"
            >{$i18nStore.get_key(
              "i18:/key-form/labels/charset",
              "Charset",
            )}</span
          >
          <select
            bind:value={selected_charset}
            class:input-error={field_errors.charset}
            class="select"
            name="charset"
            required
          >
            {#each charset_list as charsetItem (charsetItem.name)}
              <option
                selected={data.charset === charsetItem.charset}
                value={charsetItem.charset}
              >
                {charsetItem.name}
              </option>
            {/each}
          </select>
        </label>
        <div class="flex justify-end items-center mt-2 font-light text-xs">
          {selected_charset}
        </div>
        {#if field_errors.charset}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.charset as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}`,
                  error.message ?? "",
                )}
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label" for="target_size">
          <span class="font-bold">
            {$i18nStore.get_key(
              "i18:/key-form/labels/pass-length",
              "Password Length",
            )}
          </span>
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
        {#if field_errors.target_size}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.target_size as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}`,
                  error.message ?? "",
                )}
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      <div>
        <label class="label">
          <span class="font-bold"
            >{$i18nStore.get_key(
              "i18:/key-form/labels/revision",
              "Revision No",
            )}</span
          >
          <input
            class:input-error={field_errors.revision}
            class="input"
            type="number"
            name="revision"
            min="0"
            step="1"
            placeholder={$i18nStore.get_key("i18:/key-form/desc/revision", "")}
            value={data?.revision ?? 0}
          />
        </label>
        {#if field_errors.revision}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.revision as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}`,
                  error.message ?? "",
                )}
              </li>
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
        <span>{$i18nStore.get_key("i18:/generic/cancel", "Cancel")}</span>
      </button>

      <button
        disabled={submitting}
        type="button"
        class="btn variant-filled-primary"
        on:click={() => {
          submitting = true;
          on_submit().finally(() => {
            submitting = false;
          });
        }}
      >
        {#if submitting}
          <ProgressRadial width="w-6" />
        {:else}
          <span>{$i18nStore.get_key("i18:/generic/confirm", "Confirm")}</span>
        {/if}
      </button>
    </div>
  </div>
{/if}
