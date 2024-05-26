<script lang="ts">
  import type { CharsetItem, KeyRequest } from "../../api";
  import type { ModalActionResult } from "./types";
  import type { ValidationError } from "../../utils";
  import {
    getModalStore,
    InputChip,
    ProgressRadial,
    RangeSlider,
  } from "@skeletonlabs/skeleton";
  import { Log } from "../../logger";
  import { ModalAction } from "./types";
  import { Api } from "../../api";
  import { getToastStore, i18nStore } from "../../stores";
  import {
    is_error_response,
    is_null_or_empty,
    is_validation_error_response,
    or_default,
  } from "../../utils";
  import { onMount } from "svelte";

  const modal_store = getModalStore();
  const toast_store = getToastStore();
  const max_note_len: number = 200;
  const max_pass_len: number = 64;

  let charset_list: CharsetItem[] = [];
  let selected_charset: string | null = null;
  let field_errors: ValidationError<KeyRequest> = {};
  let form_element: HTMLFormElement;
  let icon_path: string | undefined = undefined;
  let slider_value: number = 32;
  let note_value: string | null = null;
  let submitting: boolean = false;

  onMount(async () => {
    try {
      charset_list = await Api.get_charsets();
      selected_charset = charset_list[0]?.charset ?? "";
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

  function on_clear_icon() {
    icon_path = undefined;
  }

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

  async function form_to_object(form: FormData): Promise<KeyRequest> {
    const domain = form.get("domain");
    const username = form.get("username");
    const charset = form.get("charset");
    const target_size = or_default(
      parseInt(form.get("target_size")?.toString() ?? ""),
      32,
    );
    const notes = form.get("notes");
    const revision = or_default(
      parseInt(form.get("revision")?.toString() ?? ""),
      1,
    );
    const tags = form.getAll("tags");

    return {
      domain: domain as string,
      charset: charset as string,
      custom_icon: icon_path,
      notes: notes as string,
      revision: revision,
      tags: tags as string[],
      target_size: target_size,
      username: username as string,
    };
  }

  async function on_submit() {
    const modal_inst = $modal_store[0];
    if (!modal_inst) {
      Log.error(
        "Submit failed. Modal component is created but unable to access modal itself.",
      );
      return;
    }

    if (!form_element) {
      Log.error("Key form ref is empty.");
      return;
    }

    if (!form_element.reportValidity()) {
      return;
    }

    try {
      const form_data = new FormData(form_element);
      const key_data = await form_to_object(form_data);

      await Api.insert_key(key_data);

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
            "i18:/key-form/errors/unknown-error",
            "Key create failed.",
          ),
        );
      }
    }
  }

  async function on_custom_icon() {
    try {
      const path = await Api.select_file(["png", "jpeg", "jpg"]);

      if (!is_null_or_empty(path)) {
        icon_path = path;
      }
    } catch (err) {
      Log.error(err);

      if (is_error_response(err)) {
        toast_store.trigger_error(
          $i18nStore.get_key(`i18:/errors/${err.code}`, err.message),
        );
      } else {
        toast_store.trigger_error(
          $i18nStore.get_key(
            "i18:/key-form/errors/unknown-error",
            "Icon selection failed.",
          ),
        );
      }
    }
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
            >{$i18nStore.get_key("i18:/key-form/labels/username", "Username")}
          </span>
          <input
            class:input-error={field_errors.username}
            class="input"
            type="text"
            name="username"
            placeholder={$i18nStore.get_key("i18:/key-form/desc/username", "")}
            required
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
              <option value={charsetItem.charset}>
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
          <span class="font-bold"
            >{$i18nStore.get_key(
              "i18:/key-form/labels/pass-length",
              "Password Length",
            )}</span
          >
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
        <label class="label" for="tags">
          <span class="font-bold"
            >{$i18nStore.get_key("i18:/key-form/labels/tags", "Tags")}</span
          >
          <InputChip
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
            value={0}
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

      <div>
        <label class="label" for="custom_icon">
          <span class="font-bold">
            {$i18nStore.get_key("i18:/key-form/labels/icon", "Custom Icon")}
          </span>
          <button
            class="input p-1 flex justify-start gap-2 items-center"
            name="custom_icon"
            on:click={on_custom_icon}
          >
            <span class="btn-sm rounded-container-token variant-filled">
              {$i18nStore.get_key("i18:/generic/select-file", "Choose a file")}
            </span>
            <span>
              {$i18nStore.get_key("i18:/key-form/desc/icon", "Upload a image")}
            </span>
          </button>
          {#if icon_path}
            <span class="font-bold inline-block py-2">
              {$i18nStore.get_key("i18:/generic/preview", "Preview")}
            </span>
            <span class="card p-3 flex flex-col justify-center items-center">
              <img
                width="128px"
                src={Api.convert_img_src(icon_path)}
                alt="icon"
              />
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
