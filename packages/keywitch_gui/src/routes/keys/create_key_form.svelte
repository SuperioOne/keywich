<script lang="ts">
  import type {CharsetItem, KeyOptions} from "$lib";
  import type {ModalActionResult} from "./types";
  import type {AutocompleteOption, PopupSettings} from "@skeletonlabs/skeleton";
  import {CloseIcon, PlusIcon} from "$lib/icons"
  import {InputChip, Autocomplete, popup, RangeSlider, getModalStore} from "@skeletonlabs/skeleton";
  import {ModalAction} from "./types";
  import {RPC} from "$lib";

  let noteValue: string;
  let selectedCharset: string;
  let selectedCharsetInfo: string;
  let sliderValue: number = 32;
  let charsetInputElement: HTMLDivElement;

  const modalStore = getModalStore();
  const maximumNoteLength: number = 200;
  const maximumPassLength: number = 64;
  const popupSettings: PopupSettings = {
    event: "focus-click",
    target: "popupAutocomplete",
    placement: "bottom"
  };

  function on_charset_select(event: CustomEvent<AutocompleteOption<string, CharsetItem>>): void {
    selectedCharset = event.detail.label;
  }

  function on_popup_close() {
    const modalResult: ModalActionResult = {
      type: ModalAction.closed
    }
    $modalStore[0].response(modalResult);
    modalStore.close()
  }

  async function on_submit(event: Event) {
    const formData = new FormData(event.target);

    const keyData: KeyOptions = {};

    const modalResult: ModalActionResult = {
      type: ModalAction.submit,
      data: keyData
    }

    $modalStore[0].response(modalResult);
    modalStore.close();
  }
</script>

{#if $modalStore[0]}
  <div class="card variant-filled-surface bg-surface-100-800-token px-16 py-8 w-full sm:w-modal flex flex-col gap-10">
    <h2 class="font-bold h2">
      {$modalStore[0].title}
    </h2>
    <form id="new_key_form" class="flex gap-5 flex-col" on:submit|preventDefault={on_submit}>

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
        <label class="label">
          <span class="font-bold">Charset</span>
          <input
            bind:this={charsetInputElement}
            class="input autocomplete"
            type="search"
            name="charset"
            placeholder="Select a charset"
            required
            bind:value={selectedCharset}
            use:popup={popupSettings}
          />
        </label>
        <div>
          {selectedCharsetInfo}
        </div>
        <div
          style:width={charsetInputElement?.clientWidth ? `${charsetInputElement.clientWidth}px` : "auto"}
          class="card variant-filled-surface p-1"
          data-popup="popupAutocomplete"
        >
          {#await RPC.get_charsets()}
            Loading charsets...
          {:then options}
            <Autocomplete
              bind:input={selectedCharset}
              options={options.map(e => ({ label: e.name, value: e.name, meta: e }))}
              on:selection={on_charset_select}
            />
          {:catch err}
            <div>{err}</div>
          {/await}
        </div>
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
        class="btn variant-soft-error"
        on:click|preventDefault={on_popup_close}
      >
        <span>
          <CloseIcon/>
        </span>
        <span>Cancel</span>
      </button>

      <button
        type="submit"
        form="new_key_form"
        class="btn variant-soft-primary"
      >
        <span>
          <PlusIcon/>
        </span>
        <span>Create</span>
      </button>
    </div>
  </div>
{/if}