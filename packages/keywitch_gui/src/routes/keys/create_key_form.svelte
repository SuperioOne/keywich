<script lang="ts">
  import type {KeyOptions} from "$lib";
  import type {ModalActionResult} from "./types";
  import {InputChip, RangeSlider, getModalStore} from "@skeletonlabs/skeleton";
  import {ModalAction} from "./types";
  import {Log, RPC} from "$lib";

  let noteValue: string;
  let sliderValue: number = 32;
  let formElement: HTMLFormElement;

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
    Log.debug(JSON.stringify([...formData.values()], undefined, 2));
    
    const keyData: KeyOptions = {};
    const newKey = await RPC.add_key(keyData);
    const modalResult: ModalActionResult = {
      type: ModalAction.submitted,
      data: newKey
    }
    modalInstance.response?.(modalResult);
    modalStore.close();
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
            class="input"
            name="domain"
            type="text"
            placeholder="Ex: gitea.com, home, my secret storage"
            required
          />
        </label>
      </div>

      <div>
        <label class="label">
          <span class="font-bold">Username</span>
          <input
            class="input"
            type="text"
            name="username"
            placeholder="Ex: me@localhost"
            required
          />
        </label>
      </div>

      <div>
        <label class="label" for="charset">
          <span class="font-bold">Charset</span>
          <select
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
      </div>

      <div>
        <label class="label">
          <span class="font-bold">Notes</span>
          <textarea
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
      </div>

      <div>
        <label class="label">
          <span class="font-bold">Revision No</span>
          <input
            class="input"
            type="number"
            name="revision"
            min="0"
            step="1"
            placeholder="(Optional) Password seed number"
          />
        </label>
      </div>

      <div>
        <label class="label">
          <span class="font-bold">Custom Icon</span>
          <input class="input" name="custom_icon" type="file"/>
        </label>
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