import type {CharsetItem, KeyManagerRpc, KeyMetadataItem, NewKeyRequest} from "$lib";

export class TestRpc implements KeyManagerRpc {
  #key_container: KeyMetadataItem[];
  #last_key: number;

  constructor() {
    this.#key_container = [
      {
        charset: "A..Za..z",
        domain: "example.com",
        notes: "test notes",
        pinned: false,
        id: 1,
        revision: 1,
        target_size: 12,
        user_name: "admin",
        createdAt: Date.now() - 100000
      },
      {
        charset: "A..Za..z",
        domain: "example-2.com",
        notes: "test notes 2",
        pinned: true,
        id: 2,
        revision: 1,
        target_size: 12,
        user_name: "admin",
        createdAt: Date.now() - 10000012
      }
    ];
    this.#last_key = this.#key_container.length;
  }

  get_charsets(): Promise<CharsetItem[]> {
    return Promise.resolve([]);
  }

  calculate_password(): Promise<string> {
    return Promise.resolve("");
  }

  get_key_collection(): Promise<KeyMetadataItem[]> {
    return Promise.resolve(this.#key_container);
  }

  get_pinned_items(): Promise<KeyMetadataItem[]> {
    return Promise.resolve(this.#key_container.filter(e => e.pinned));
  }

  add_key(request: NewKeyRequest): Promise<KeyMetadataItem> {
    const newItem: KeyMetadataItem = {
      ...request,
      createdAt: Date.now(),
      id: this.#last_key + 1,
      pinned: false
    }
    this.#key_container.push(newItem);
    this.#last_key += 1;

    return Promise.resolve(newItem);
  }

  remove_key(id: number): Promise<boolean> {
    this.#key_container = this.#key_container.filter(e => e.id !== id);
    return Promise.resolve(true);
  }
}