import RPC from "@keywitch/memory_rpc";
import type {KeyMetadataItem} from "@keywitch/rpc";
import {AdvancedCopyMenu, KeyForm, ModalAction} from "$lib/components";
import {Log} from "$lib/logger";
import {getExtendedToastStore} from "$lib/stores";
import {getModalStore} from "@skeletonlabs/skeleton";
import {goto} from "$app/navigation";
import {i18nStore, type ModalActionResult, type TokenType} from "$lib";

// All app actions which can be called via UI elements, commands, keyboard shortcuts

export type EventDispatcher = {
  new_key: () => Promise<KeyMetadataItem | undefined>;
  quick_copy: (item: KeyMetadataItem) => Promise<boolean>;
  advanced_copy: (item: KeyMetadataItem) => Promise<boolean>;
  update_key: (item: KeyMetadataItem) => Promise<KeyMetadataItem | undefined>;
  delete_key: (item: KeyMetadataItem) => Promise<boolean>;
  flip_pin: (item: KeyMetadataItem) => Promise<boolean>;
  search_keys: (tokens: TokenType[]) => Promise<void>;
};

export function create_event_manager(): EventDispatcher {
  const modalStore = getModalStore();
  const toastStore = getExtendedToastStore();

  async function new_key(): Promise<KeyMetadataItem | undefined> {
    const response = await new Promise<ModalActionResult>((resolve) => {
      modalStore.trigger({
        component: {
          ref: KeyForm,
        },
        title: i18nStore.getKey("i18:/actions/new-key/title", "Create New Key"),
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult) => resolve(r),
      });
    });

    if (response?.type === ModalAction.submitted) {

      toastStore.trigger_success(
        i18nStore.getKey(
          `i18:/actions/new-key/msg/success?$noCache&domain=${response.data.domain}`,
          "New key created.")
      );
      return response.data;
    }

    return undefined;
  }

  async function update_key(item: KeyMetadataItem): Promise<KeyMetadataItem | undefined> {
    const response = await new Promise<ModalActionResult>((resolve) => {
      modalStore.trigger({
        component: {
          ref: KeyForm,
          props: {
            data: item
          }
        },
        title: i18nStore.getKey("i18:/actions/update-key/title", "Update Key"),
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult) => resolve(r),
      });
    });

    if (response?.type === ModalAction.submitted) {
      toastStore.trigger_success(
        i18nStore.getKey(
          `i18:/actions/update-key/msg/success`,
          "Key updated successfully.")
      );
      return response.data;
    }

    return undefined;
  }

  async function flip_pin(item: KeyMetadataItem) {
    const rpcAction = item.pinned
      ? RPC.KeyMetadata.unpin_key
      : RPC.KeyMetadata.pin_key;

    const result = await rpcAction(item.id);

    if (result.success) {
      return true;
    }

    Log.error(result.error);
    toastStore.trigger_error(
      i18nStore.getKey(
        `i18:/actions/pin-key/msg/error`,
        "Unable to pin")
    );
    return false;
  }

  async function quick_copy(item: KeyMetadataItem) {
    try {
      const result = await RPC.KeyMetadata.generate_password(item.id, "text");

      if (result.success) {
        await navigator.clipboard.writeText(result.data);
        toastStore.trigger_success(
          i18nStore.getKey(
            `i18:/actions/copy-key/msg/success`,
            "Key copied.")
        );
        return true;
      }

      Log.error(result.error);
    } catch (err) {
      Log.error(err);
    }

    toastStore.trigger_error(
      i18nStore.getKey(
        `i18:/actions/copy-key/msg/error`,
        "Key generation failed.")
    );
    return false;
  }

  async function advanced_copy(item: KeyMetadataItem) {
    try {
      const result = await new Promise<ModalActionResult>((resolve) => {
        modalStore.trigger({
          component: {
            ref: AdvancedCopyMenu,
            props: {
              keyId: item.id
            }
          },
          title: i18nStore.getKey(`i18:/actions/advanced-copy/title`, "Advanced"),
          backdropClasses: "backdrop-blur-sm",
          type: "component",
          response: (r: ModalActionResult) => resolve(r),
        });
      });

      return true;
    } catch (err) {
      Log.error(err);
      return false;
    }
  }

  async function delete_key(item: KeyMetadataItem) {
    const confirmation = await new Promise((resolve) => {
      modalStore.trigger({
        type: "confirm",
        title: i18nStore.getKey("i18:/actions/delete-key/title", "Confirm Action"),
        body: i18nStore.getKey(
          `i18:/actions/delete-key/message?$noCache&username=${item.user_name}&domain=${item.domain}`,
          "Are you sure to delete key?"
        ),
        buttonTextConfirm: i18nStore.getKey("i18:/generic/delete", "Delete"),
        buttonTextCancel: i18nStore.getKey("i18:/generic/cancel", "Cancel"),
        response: (r: boolean) => resolve(r),
      });
    });

    if (confirmation) {
      const result = await RPC.KeyMetadata.remove_key(item.id);

      if (result.success) {
        toastStore.trigger_warning(i18nStore.getKey("i18:/actions/delete-key/msg/success", "Key deleted from store."));
        return true;
      }

      Log.warn(result.error);
      toastStore.trigger_error(i18nStore.getKey("i18:/actions/delete-key/msg/error", "Unable to delete key."));
    }

    return false;
  }

  async function search_keys(tokens: TokenType[]) {
    const target = new URL("/keys", document.location.origin);

    if (tokens && tokens.length > 0) {
      let name: string;

      for (const searchQuery of tokens) {
        switch (searchQuery.type) {
          case "username":
            name = "u";
            break;
          case "domain":
            name = "d";
            break;
          case "tag":
            name = "t";
            break;
          case "term":
            name = "s";
            break;
        }

        target.searchParams.append(name, searchQuery.value);
      }
    }

    await goto(target, {
      invalidateAll: true,
      keepFocus: true,
    });
  }

  return {
    new_key,
    update_key,
    delete_key,
    flip_pin,
    advanced_copy,
    quick_copy,
    search_keys
  }
}
