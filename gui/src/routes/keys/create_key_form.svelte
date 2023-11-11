<script lang="ts">
  import type {ErrorType, KeyOptions} from "$lib";
  import type {ModalActionResult} from "./types";
  import {InputChip, RangeSlider, getModalStore} from "@skeletonlabs/skeleton";
  import {ModalAction} from "./types";
  import {Log, RPC} from "$lib";
  import {or_default} from "$lib/utils/or_default.js";

  let noteValue: string;
  let sliderValue: number = 32;
  let formElement: HTMLFormElement;
  let errors: ErrorType<KeyOptions> = {};

  const modalStore = getModalStore();
  const maximumNoteLength: number = 200;
  const maximumPassLength: number = 64;

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
    const addResult = await RPC.add_key(keyData);

    if (addResult.success) {
      const modalResult: ModalActionResult = {
        type: ModalAction.submitted,
        data: addResult.data
      }
      modalInstance.response?.(modalResult);
      modalStore.close();
    } else {
      errors = addResult.errors;
    }
  }
</script>

{#if $modalStore[0]}
  <div class="card variant-filled-surface bg-surface-100-800-token px-16 py-8 w-full sm:w-modal flex flex-col gap-10">
    <h2 class="font-bold h2">
      {$modalStore[0].title}
    </h2>
    <form bind:this={formElement} id="new_key_form" class="flex gap-5 flex-col" on:submit|preventDefault={on_submit}>

      <div>
        <label class="label">
          <span class="font-bold">Domain</span>
          <input
            class:input-error={errors.domain}
            class="input"
            name="domain"
            type="text"
            placeholder="Ex: gitea.com, home, my secret storage"
            required
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
          <span class="font-bold">Username</span>
          <input
            class:input-error={errors.user_name}
            class="input"
            type="text"
            name="user_name"
            placeholder="Ex: me@localhost"
            required
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
          <span class="font-bold">Charset</span>
          <select
            class:input-error={errors.charset}
            class="select"
            name="charset"
            required
          >
            {#await RPC.get_charsets()}
              <option disabled>Loading...</option>
            {:then charsetCollection}
              {#each charsetCollection as charsetItem (charsetItem.name)}
                <option value={charsetItem.charset}>{charsetItem.name}</option>
              {/each}
            {:catch err}
              <div>{err}</div>
            {/await}
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
          <span class="font-bold">Password Length</span>
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
          <span class="font-bold">Tags</span>
          <InputChip
            name="tags"
            chips="variant-filled-primary"
            placeholder="(Optional) Enter tags"
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
          <span class="font-bold">Notes</span>
          <textarea
            class:input-error={errors.notes}
            class="textarea"
            rows="4"
            name="notes"
            placeholder="(Optional) Notes about the key"
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
          <span class="font-bold">Revision No</span>
          <input
            class:input-error={errors.revision}
            class="input"
            type="number"
            name="revision"
            min="0"
            step="1"
            placeholder="(Optional) Password seed number"
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
        <label class="label">
          <span class="font-bold">Custom Icon</span>
          <input
            class:input-error={errors.custom_icon}
            class="input"
            name="custom_icon"
            type="file"
          />
        </label>
        {#if errors.custom_icon}
          <ul
            class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside"
          >
            {#each errors.custom_icon as error}
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
        <span>Cancel</span>
      </button>

      <button
        type="submit"
        form="new_key_form"
        class="btn variant-soft-primary"
      >
        <span>Create</span>
      </button>
    </div>
  </div>
{/if}