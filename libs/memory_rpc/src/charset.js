import {parse_CharsetOptions} from "@keywitch/rpc/parsers";

/**
 * @type {import("@keywitch/rpc").CharsetItem[]}
 */
const MEMORY_STORE = [
  {
    charset: "A..Za..z0..9",
    id: 1,
    name: "alpha numeric",
    description: "ASCII compliant alpha numeric character set"
  },
  {
    charset: "A..Za..z",
    id: 2,
    name: "alpha",
    description: "ASCII compliant alpha character set"
  },
  {
    charset: "0..9",
    id: 3,
    name: "numeric",
    description: "Numeric only character set"
  }
];
let NEXT_ID = 4;

/**
 * @type {import("@keywitch/rpc").CharsetRPC}
 */
const module = {
  create_charset(charset) {
    const validationResult = parse_CharsetOptions(charset);

    if (validationResult?.success === false) return Promise.resolve(validationResult);

    /**
     * @type {import("@keywitch/rpc").CharsetItem}
     */
    const newCharset = {
      ...charset,
      id: NEXT_ID
    };
    NEXT_ID++
    MEMORY_STORE.push(newCharset);

    return Promise.resolve({
      success: true,
      data: structuredClone(newCharset)
    });
  },
  get_charsets() {
    return Promise.resolve({
      success: true,
      data: structuredClone(MEMORY_STORE)
    });
  },
  remove_charset(id) {
    const index = MEMORY_STORE.findIndex(e => e.id === id);

    if (index > -1) {
      MEMORY_STORE.splice(index, 1);
      return Promise.resolve({
        success: true
      });
    } else {
      return Promise.resolve({
        success: false,
        error: "Charset item does not exists"
      });
    }
  }
};

export default module;