<script lang="ts">
  import KeywichIcon from "$lib/icons/keywich.svelte"
  import type {PageData} from "./$types";
  import type {ValidationError} from "$lib";
  import {ProgressRadial} from "@skeletonlabs/skeleton";
  import {getToastStore, i18nStore, Log, RPC} from "$lib";
  import {goto} from "$app/navigation";
  import {is_null_or_empty} from "@keywich/api/utils";


  type PassForm = { password?: string, re_password?: string };

  export let data: PageData;

  const toast_store = getToastStore();
  let field_errors: ValidationError<PassForm> = {};
  let unlocking: boolean = false;

  async function on_submit(event: Event) {
    const form_element = event.target as HTMLFormElement;
    const form_data = new FormData(form_element);
    const pass = form_data.get("password")?.toString();
    const re_pass = form_data.get("re_password")?.toString();

    if (!form_element.reportValidity()) {
      return false;
    }

    const {errors, is_valid} = validate({
      password: pass,
      re_password: re_pass
    });

    if (!is_valid) {
      field_errors = errors;
      return false;
    }

    if (!is_null_or_empty(pass)) {
      try {
        await RPC.login(pass);
        sessionStorage.setItem("unlocked", "1");
        await goto("/");
      } catch (err) {
        Log.error(err);
        toast_store.trigger_error($i18nStore.get_key("i18:/unlock/db-error", "Unable to unlock."));
        form_element.reset();
      }
    } else {
      Log.warn("Validation passed but pass value is still invalid!");
    }
  }

  function validate(form_data: PassForm) {
    let errors: ValidationError<PassForm> = {};
    let is_valid = true;

    if (is_null_or_empty(form_data.password)) {
      is_valid = false;
      errors.password = [{
        code: "length",
        message: "Password cannot be empty",
        params: {}
      }];
    }

    if (!data.is_db_created) {
      if (is_null_or_empty(form_data.re_password)) {
        is_valid = false;
        errors.re_password = [{
          code: "length",
          message: "Password cannot be empty",
          params: {}
        }];
      }

      if (form_data.re_password !== form_data.password) {
        is_valid = false;
        errors.re_password = errors.re_password ?? [];

        errors.re_password.push({
          code: "password_must_match",
          message: "Passwords does not match.",
          params: {}
        });
      }
    }

    return {is_valid, errors};
  }
</script>

<div class="flex justify-center w-full">
  <div class="p-3 sm:py-8 sm:px-34 md:py-16 md:px-48 w-full max-w-screen-lg ">
    <div class="flex flex-row justify-center mb-4 fill-primary-500 drop-shadow-lg">
      <KeywichIcon size={300}/>
    </div>

    <form class="flex flex-col gap-10"
          on:submit|preventDefault={async (e) => {unlocking = true;on_submit(e).finally(()=> { unlocking = false});}}>
      <label class="label">
        <span class="font-bold">{$i18nStore.get_key("i18:/unlock/label/password", "Master Password")}</span>
        <input
            class:input-error={field_errors.password}
            class="input"
            name="password"
            type="password"
            placeholder={$i18nStore.get_key("i18:/unlock/desc/password", "")}
            required
        />
        {#if field_errors.password}
          <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
            {#each field_errors.password as error}
              <li>
                {$i18nStore.get_key(`i18:/field-errors/${error.code}`, error.message ?? "")}
              </li>
            {/each}
          </ul>
        {/if}
      </label>

      <label class="label" class:hidden={data.is_db_created}>
        <span class="font-bold">{$i18nStore.get_key("i18:/unlock/label/re-password", "Re-Enter Password")}</span>
        <input
            class:input-error={field_errors.re_password}

            disabled={data.is_db_created}
            class="input"
            name="re_password"
            type="password"
            placeholder={$i18nStore.get_key("i18:/unlock/desc/re-password", "")}
            required
        />
        {#if field_errors.re_password}
          <ul class="m-1 font-light text-sm text-error-500-400-token list-disc list-inside">
            {#each field_errors.re_password as error}
              <li>
                {$i18nStore.get_key(`i18:/field-errors/${error.code}`, error.message ?? "")}
              </li>
            {/each}
          </ul>
        {/if}
      </label>

      <button
          disabled={unlocking}
          type="submit"
          class="btn variant-filled-primary"
      >
        {#if unlocking}
          <ProgressRadial width="w-6"/>
        {:else}
          {#if data.is_db_created}
            <span>{$i18nStore.get_key("i18:/unlock/open", "Unlock")}</span>
          {:else }
            <span>{$i18nStore.get_key("i18:/unlock/create", "Create")}</span>
          {/if}
        {/if}
      </button>
    </form>
  </div>
</div>