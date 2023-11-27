import RPC from "@keywitch/memory_rpc";
import type {KeyMetadataItem} from "@keywitch/rpc";
import type {ModalActionResult} from "$lib";
import {AdvancedCopyMenu, KeyForm, ModalAction} from "$lib/components";
import {Log} from "$lib/logger";
import {getExtendedToastStore} from "$lib/stores";
import {getModalStore} from "@skeletonlabs/skeleton";

export type EventDispatcher = {
  new_key: () => Promise<KeyMetadataItem | undefined>;
  quick_copy: (item: KeyMetadataItem) => Promise<boolean>;
  advanced_copy: (item: KeyMetadataItem) => Promise<boolean>;
  update_key: (item: KeyMetadataItem) => Promise<KeyMetadataItem | undefined>;
  delete_key: (item: KeyMetadataItem) => Promise<boolean>;
  flip_pin: (item: KeyMetadataItem) => Promise<boolean>
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
        title: "Create New Key",
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult) => resolve(r),
      });
    });

    if (response?.type === ModalAction.submitted) {
      toastStore.trigger_success("New key created successfully.");
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
        title: "Update Key",
        backdropClasses: "backdrop-blur-sm",
        type: "component",
        response: (r: ModalActionResult) => resolve(r),
      });
    });

    if (response?.type === ModalAction.submitted) {
      toastStore.trigger_success("Key updated successfully.");
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
    toastStore.trigger_error("Unable to pin");
    return false;
  }

  async function quick_copy(item: KeyMetadataItem) {
    try {
      const result = await RPC.KeyMetadata.generate_password(item.id, "text");

      if (result.success) {
        await navigator.clipboard.writeText(result.data);
        toastStore.trigger_success("Key copied to clipboard.");
        return true;
      }

      Log.error(result.error);
    } catch (err) {
      Log.error(err);
    }

    toastStore.trigger_error("Key generation failed. See logs for more details.");
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
          title: "Advanced options",
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
        title: "Confirm Action",
        body: `Are you sure to delete key?`,
        buttonTextConfirm: "Delete",
        response: (r: boolean) => resolve(r),
      });
    });

    if (confirmation) {
      const result = await RPC.KeyMetadata.remove_key(item.id);

      if (result.success) {
        toastStore.trigger_warning("Key removed from store.");
        return true;
      }

      Log.warn(result.error);
      toastStore.trigger_error("Unable to remove key.");
    }

    return false;
  }

  return {
    new_key,
    update_key,
    delete_key,
    flip_pin,
    advanced_copy,
    quick_copy
  }
}
