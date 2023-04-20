<script lang="ts">
  import { ClipboardIcon, TrashIcon, KeyIcon, StarIcon } from "$lib/icons";
  import { InputChip } from '@skeletonlabs/skeleton';

  const tableArr: any[] = [];
  let list: string[] = ['foo', 'bar', 'fizz', 'buzz'];
  for (let i = 0; i < 50; i++) {
    tableArr.push({
      preset: `preset${i}`,
      site: `site${i}`,
      userName: "username",
    });
  }
</script>

<div class="flex gap-6 flex-col">
  <div>
    <InputChip bind:value={list} name="chips" chips="chip font-bold text-xs variant-soft-secondary" placeholder="Filter by tags" />
  </div>

  <div class="card p-4">
    <dl class="list-dl">
      {#each tableArr as row, i (i)}
        <div class="gap-1 justify-between">
          <span
            class="flex-none text-primary-500 bg-surface-200-700-token p-2 btn-sm btn"
          >
            <KeyIcon />
          </span>
          <span class="flex-auto">
            <dt class="font-bold">{row.userName}</dt>
            <dd class="text-sm font-thin">{row.site}</dd>
          </span>
          <span class="flex-1 shrink flex flex-wrap gap-1">
              <span class="chip font-bold text-xs variant-soft-secondary">
                Preset
              </span>
              <span class="chip font-bold text-xs variant-soft-secondary">
                tag 1
              </span>
              <span class="chip font-bold text-xs variant-soft-secondary">
                tag 2
              </span>
              <span class="chip font-bold text-xs variant-soft-secondary">
                tag 3
              </span>
          </span>
          <span class="flex-none">
              <button
                type="button"
                class="btn btn-sm variant-soft btn-icon-base"
              >
                <StarIcon />
              </button>
              <button
                type="button"
                class="btn btn-sm variant-soft-secondary btn-icon-base"
              >
                <ClipboardIcon />
              </button>
              <button
                type="button"
                class="btn btn-sm variant-soft-error btn-icon-base"
              >
                <TrashIcon />
              </button>
          </span>
        </div>
        {#if i !== tableArr.length - 1}
          <hr />
        {/if}
      {/each}
    </dl>
  </div>
</div>
