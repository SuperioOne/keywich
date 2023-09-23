<script lang="ts">
  import type {ModalActionResult} from "./types";
  import type {PopupSettings} from "@skeletonlabs/skeleton";
  import {CloseIcon, PlusIcon} from "$lib/icons"
  import {InputChip, Autocomplete, popup, RangeSlider, getModalStore} from "@skeletonlabs/skeleton";
  import {ModalAction} from "./types";

  const options = [
    {label: 'Vanilla', value: 'vanilla', keywords: 'plain, basic', meta: {healthy: false}},
    {label: 'Chocolate', value: 'chocolate', keywords: 'dark, white', meta: {healthy: false}},
    {label: 'Strawberry', value: 'strawberry', keywords: 'fruit', meta: {healthy: true}},
    {label: 'Neapolitan', value: 'neapolitan', keywords: 'mix, strawberry, chocolate, vanilla', meta: {healthy: false}},
    {label: 'Pineapple', value: 'pineapple', keywords: 'fruit', meta: {healthy: true}},
    {label: 'Peach', value: 'peach', keywords: 'fruit', meta: {healthy: true}}
  ];

  let noteValue: string;
  let inputPopupDemo: string;
  let sliderValue: number = 32;

  const modalStore = getModalStore();
  const maximumNoteLength: number = 200;
  const maximumPassLength: number = 64;
  const popupSettings: PopupSettings = {
    event: 'focus-click',
    target: 'popupAutocomplete',
    placement: 'bottom',
  };

  function onPopupDemoSelect(event: CustomEvent<FlavorOption>): void {
    inputPopupDemo = event.detail.label;
  }

  function onPopupClose() {
    const modalResult: ModalActionResult = {
      type: ModalAction.closed
    }
    $modalStore[0].response(modalResult);
    modalStore.close()
  }

  async function onSubmit(event: Event) {
    const modalResult: ModalActionResult = {
      type: ModalAction.submit,
      data: new FormData(event.target)
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
    <form id="new_key_form" class="flex gap-5 flex-col" on:submit|preventDefault={onSubmit}>

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
            class="input autocomplete"
            type="search"
            name="charset"
            placeholder="Select a charset"
            required
            bind:value={inputPopupDemo}
            use:popup={popupSettings}
          />
        </label>
        <div class="card variant-filled-surface" data-popup="popupAutocomplete">
          <Autocomplete
            bind:input={inputPopupDemo}
            options={options}
            on:selection={onPopupDemoSelect}
          />
        </div>
      </div>

      <div>
        <label class="label">
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
        <label class="label">
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
        class="btn variant-filled-error"
        on:click|preventDefault={onPopupClose}
      >
        <span>
          <CloseIcon/>
        </span>
        <span>Cancel</span>
      </button>

      <button
        type="submit"
        form="new_key_form"
        class="btn variant-filled-primary"
      >
        <span>
          <PlusIcon/>
        </span>
        <span>Create</span>
      </button>
    </div>
  </div>
{/if}