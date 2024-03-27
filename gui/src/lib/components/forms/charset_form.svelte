<script lang="ts">
  import type { ModalActionResult } from "./types";
  import type { CharsetOptions, CharsetItem } from "../../api";
  import { getModalStore } from "@skeletonlabs/skeleton";
  import { Log } from "../../logger";
  import { ModalAction } from "./types";
  import { Api } from "../../api";
  import type { ValidationError } from "../../utils";
  import { getToastStore, i18nStore } from "../../stores";
  import { is_error_response, is_validation_error_response } from "../../utils";

  let field_errors: ValidationError<CharsetOptions> = {};
  let form_element: HTMLFormElement;
  let submitting: boolean = false;

  const modal_store = getModalStore();
  const toast_store = getToastStore();

  function on_popup_close() {
    const modal = $modal_store[0];
    if (!modal) {
      Log.error(
        new Error(
          "Close action failed. Modal component is created but unable to access modal itself.",
        ),
      );
      return;
    }

    const response: ModalActionResult<CharsetItem> = {
      type: ModalAction.closed,
    };
    modal.response?.(response);
    modal_store.close();
  }

  function form_to_object(form: FormData) {
    const charset = form.get("charset");
    const name = form.get("name");
    const description = form.get("description");

    return {
      charset,
      name,
      description,
    };
  }

  async function on_submit() {
    const modal = $modal_store[0];
    if (!modal) {
      Log.error(
        "Submit failed. Modal component is created but unable to access modal itself.",
      );
      return;
    }

    if (!form_element) {
      Log.error("Charset form ref is empty.");
      return;
    }

    if (!form_element.reportValidity()) {
      return;
    }

    const form_data = new FormData(form_element);
    const charset = form_to_object(form_data) as unknown as CharsetOptions;

    try {
      let result = await Api.insert_charset(charset);

      const response: ModalActionResult<string> = {
        type: ModalAction.submitted,
        data: result,
      };

      modal.response?.(response);
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
            "i18:/charset-form/unknown-error",
            "Charset save failed.",
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
      id="charset_form"
      class="flex gap-5 flex-col"
      on:submit|preventDefault={on_submit}
    >
      <div>
        <label class="label">
          <span class="font-bold"
            >{$i18nStore.get_key("i18:/charset-form/labels/name", "Name")}</span
          >
          <input
            class:input-error={field_errors.name}
            class="input"
            name="name"
            type="text"
            placeholder={$i18nStore.get_key("i18:/charset-form/desc/name", "")}
            required
          />
        </label>
        {#if field_errors.name}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.name as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}?field=name&$noCache`,
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
              "i18:/charset-form/labels/charset",
              "Charset",
            )}</span
          >
          <input
            class:input-error={field_errors.charset}
            class="input"
            type="text"
            name="charset"
            placeholder={$i18nStore.get_key(
              "i18:/charset-form/desc/charset",
              "",
            )}
            required
          />
        </label>
        {#if field_errors.charset}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.charset as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}?field=charset&$noCache`,
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
              "i18:/charset-form/labels/description",
              "Description",
            )}</span
          >
          <input
            class:input-error={field_errors.description}
            class="input"
            type="text"
            name="description"
            placeholder={$i18nStore.get_key(
              "i18:/charset-form/desc/description",
              "",
            )}
          />
        </label>
        {#if field_errors.description}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each field_errors.description as error}
              <li>
                {$i18nStore.get_key(
                  `i18:/field-errors/${error.code}?field=description&$noCache`,
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
        type="button"
        class="btn variant-filled-primary"
        on:click={() => {
          submitting = true;
          on_submit().finally(() => {
            submitting = false;
          });
        }}
      >
        <span>{$i18nStore.get_key("i18:/generic/confirm", "Confirm")}</span>
      </button>
    </div>
  </div>
{/if}
