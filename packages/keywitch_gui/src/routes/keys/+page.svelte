<script lang="ts">
  import {
    ClipboardIcon,
    TrashIcon,
    KeyIcon,
    PlusCircleIcon,
    StarIcon,
    EditIcon,
  } from "$lib/icons";
  import { InputChip } from "@skeletonlabs/skeleton";

  const tableArr: any[] = [];
  let list: string[] = [];
  for (let i = 0; i < 50; i++) {
    tableArr.push({
      preset: `preset${i}`,
      site: `https://en.wikipedia.org/wiki/Bcryptasdasdadadasdadasdasdadasdadasda${i}`,
      userName: "username",
    });
  }
</script>

<div class="flex gap-6 flex-col">
  <div>
    <InputChip
      bind:value={list}
      name="filter"
      chips="chip font-bold text-xs variant-soft-secondary"
      placeholder="filter..."
    />
  </div>

  <div class="flex flex-row justify-center sm:justify-end gap-1">
    <button type="button" class="btn variant-filled-primary w-full sm:w-auto"
      ><PlusCircleIcon />
      <span class="font-bold"> New </span>
    </button>
  </div>

  <div class="card p-1 sm:p-4">
    <dl>
      {#each tableArr as row, i (i)}
        <div class="grid grid-cols-4 gap-3 w-full p-3">
          <div class="col-span-3 flex flex-row sm:gap-8 gap-3 items-center">
            <div
              class="text-primary-500 bg-surface-200-700-token p-3 btn-sm btn h-fit"
            >
              <KeyIcon />
            </div>
            <div class="w-full overflow-hidden flex flex-col gap-3">
              <div>
                <dt class="font-bold">{row.userName}</dt>
                <dd class="text-sm font-thin truncate">
                  {row.site}
                </dd>
              </div>
              <div class="flex flex-wrap gap-1">
                <span
                  class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
                >
                  Preset
                </span>
                <span
                  class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
                >
                  tag 1
                </span>
                <span
                  class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
                >
                  tag 2
                </span>
                <span
                  class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
                >
                  tag 3
                </span>
              </div>
            </div>
          </div>
          <div
            class="flex flex-col md:flex-row flex-wrap gap-1 sm:gap-2 justify-end items-center"
          >
            <button
              type="button"
              class="btn btn-sm variant-soft-secondary btn-icon-base h-fit"
            >
              <ClipboardIcon />
            </button>
            <button
              type="button"
              class="btn btn-sm variant-soft btn-icon-base h-fit"
            >
              <StarIcon />
            </button>
            <button
              type="button"
              class="btn btn-sm variant-soft btn-icon-base h-fit"
            >
              <EditIcon />
            </button>
            <button
              type="button"
              class="btn btn-sm variant-soft-error btn-icon-base h-fit"
            >
              <TrashIcon />
            </button>
          </div>
        </div>

        {#if i !== tableArr.length - 1}
          <hr />
        {/if}
      {/each}
    </dl>
  </div>
</div>
