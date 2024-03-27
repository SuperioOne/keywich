<script lang="ts">
  import { invalidateAll } from "$app/navigation";
  import { i18nStore, Api, Log, is_error_response, getToastStore } from "$lib";
  import ExportIcon from "$lib/icons/download.svelte";
  import ImportIcon from "$lib/icons/upload.svelte";
  import { ProgressRadial, getModalStore } from "@skeletonlabs/skeleton";

  const toast_store = getToastStore();
  const modal_store = getModalStore();
  let exporting: boolean = false;
  let restoring: boolean = false;

  async function export_profile_db() {
    try {
      await Api.backup();

      toast_store.trigger_success(
        $i18nStore.get_key(
          "i18:/settings/backup/export/success",
          "Profile database exported.",
        ),
      );
    } catch (err) {
      Log.error(err);

      if (is_error_response(err)) {
        toast_store.trigger_error(
          $i18nStore.get_key(`i18:/errors/${err.code}`, err.message),
        );
      }
    }
  }

  async function import_profile_db() {
    try {
      const result = await Api.verify_backup();

      if (!result.is_valid) {
        const confirmation = await new Promise((resolve) => {
          modal_store.trigger({
            type: "confirm",
            title: $i18nStore.get_key(
              "i18:/settings/backup/restore/invalid-signature/title",
              "Unmatched Backup Signature",
            ),
            body: $i18nStore.get_key(
              `i18:/settings/backup/restore/invalid-signature/desc`,
              "Are you sure about the restoring the backup file?",
            ),
            buttonTextConfirm: $i18nStore.get_key(
              "i18:/generic/confirm",
              "Confirm",
            ),
            buttonTextCancel: $i18nStore.get_key(
              "i18:/generic/cancel",
              "Cancel",
            ),
            response: (r: boolean) => resolve(r),
          });
        });

        if (!confirmation) {
          return;
        }
      }

      await Api.restore(result.path);

      toast_store.trigger_success(
        $i18nStore.get_key(
          "i18:/settings/backup/restore/success",
          "Profile database restored.",
        ),
      );

      await invalidateAll();
    } catch (err) {
      Log.error(err);

      if (is_error_response(err)) {
        toast_store.trigger_error(
          $i18nStore.get_key(`i18:/errors/${err.code}`, err.message),
        );
      }
    }
  }
</script>

<div class="flex flex-col gap-8">
  <div
    class="flex flex-row flex-wrap sm:flex-nowrap justify-between gap-2 w-full sm:w-auto"
  >
    <div>
      <h2 class="font-bold">
        {$i18nStore.get_key("i18:/settings/backup/export/title", "Backup")}
      </h2>
      <p class="font-light">
        <small>
          {$i18nStore.get_key(
            "i18:/settings/backup/export/desc",
            "Backup profile database and icons",
          )}
        </small>
      </p>
    </div>
    <button
      class="btn flex justify-around variant-filled-primary max-w-[150px] w-full"
      on:click={() => {
        exporting = true;
        export_profile_db().finally(() => {
          exporting = false;
        });
      }}
      disabled={exporting}
    >
      {#if exporting}
        <ProgressRadial width="w-6" />
      {:else}
        <ExportIcon size={20} />
      {/if}
      <span>
        {$i18nStore.get_key("i18:/settings/backup/export", "Backup")}
      </span>
    </button>
  </div>

  <div
    class="flex flex-row flex-wrap sm:flex-nowrap justify-between gap-2 w-full sm:w-auto"
  >
    <div>
      <h2 class="font-bold">
        {$i18nStore.get_key("i18:/settings/backup/restore/title", "Restore")}
      </h2>
      <p class="font-light">
        <small>
          {$i18nStore.get_key(
            "i18:/settings/backup/restore/desc",
            "Restore profile from a backup file",
          )}
        </small>
      </p>
    </div>
    <button
      class="btn flex justify-around variant-filled-primary max-w-[150px] w-full"
      on:click={() => {
        restoring = true;
        import_profile_db().finally(() => {
          restoring = false;
        });
      }}
      disabled={restoring}
    >
      {#if restoring}
        <ProgressRadial width="w-6" />
      {:else}
        <ImportIcon size={20} />
      {/if}
      <span>
        {$i18nStore.get_key("i18:/settings/backup/restore", "Restore")}
      </span>
    </button>
  </div>
</div>
