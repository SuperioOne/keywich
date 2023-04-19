<script lang="ts">
  import { ClipboardIcon, SearchIcon, TrashIcon, KeyIcon } from "$lib/icons";
  import { focusTrap } from "@skeletonlabs/skeleton";

  const tableArr: any[] = [];
  let inputDemo = "";

  for (let i = 0; i < 10; i++) {
    tableArr.push({
      preset: `preset${i}`,
      site: `site${i}`,
      userName: "username",
    });
  }
</script>

<div class="flex flex-col gap-5">
  <div
    use:focusTrap={true}
    class="input-group input-group-divider grid-cols-[auto_1fr_auto]"
  >
    <div class="input-group-shim">
      <SearchIcon />
    </div>
    <input type="search" bind:value={inputDemo} placeholder="Filter" />
  </div>

  <dl class="list-dl">
    {#each tableArr as row, i}
      <div class="flex flex-row justify-between">
        <div>
          <span class="text-primary-500"> <KeyIcon /></span>
          <span>
            <dd>{row.userName}</dd>
            <dt class="sub-text">{row.site}</dt>
          </span>
        </div>
        <div>
          <span>
            <dd>
              {row.preset}
            </dd>
          </span>
        </div>
        <div>
          <button type="button" class="btn btn-sm variant-ghost btn-icon-base">
            <ClipboardIcon />
          </button>
          <button
            type="button"
            class="btn btn-sm variant-ghost-warning btn-icon-base"
          >
            <TrashIcon />
          </button>
        </div>
      </div>
      <hr />
    {/each}
  </dl>
</div>

<style>
  .sub-text {
    font-size: 0.8rem;
    color: var(--theme-font-color-base);
  }
</style>
